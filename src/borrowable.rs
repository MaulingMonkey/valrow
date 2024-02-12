/// A type that is borrowable by value.
///
/// ### Safety
/// By implementing this trait, you assert that it's safe and sound for a single instance of `Self` to exist at multiple addresses simultaniously.
/// Additionally, `Self` and <code>Self::[Abi](Self::Abi)</code> must be ABI compatible.
///
/// Do not implement this on types with direct interior mutability.
/// The copies of the "single" instance will decohere.
/// Also, <code>[Abi](Self::Abi): [Copy]</code>, but the standard library currently provides no [`Copy`]able types that also implement interior mutability.
/// This means *you* can't implement a [`Copy`]able type that implements interior mutability.
/// This means implementing the trait would violate your promise of `Self` and `Abi` being ABI compatible.
///
/// ```rust
/// # use valrow::*;
/// # use std::cell::*;
/// # use std::mem::*;
/// # use std::ptr::*;
/// # use std::rc::*;
/// #
/// #[repr(C)] pub struct A(           usize  ); // ✔️      no  interior mutability
/// #[repr(C)] pub struct B(Rc<        usize >); // ✔️ indirect interior mutability (refcounts)
/// #[repr(C)] pub struct C(Rc<RefCell<usize>>); // ✔️ indirect interior mutability (+ RefCell)
/// #[repr(C)] pub struct D(      Cell<usize> ); // ❌   direct interior mutability
/// #[repr(C)] pub struct Z(             ()   ); // ✔️      no  interior mutability
///
/// unsafe impl BorrowableByValue for A { type Abi =                 usize  ; } // ✔️ sound
/// unsafe impl BorrowableByValue for B { type Abi = NonNull<        usize >; } // ✔️ sound
/// unsafe impl BorrowableByValue for C { type Abi = NonNull<RefCell<usize>>; } // ✔️ sound
/// unsafe impl BorrowableByValue for D { type Abi =                 usize  ; } // ❌ unsound
/// unsafe impl BorrowableByValue for Z { type Abi =                   ()   ; } // ✔️ sound
/// #
/// # impl Clone for B { fn clone(&self) -> Self { Self(self.0.clone()) } }
/// # let b = B(Rc::new(42));
/// # let b_borrow_1 = Valrow::new(&b); // doesn't Clone B nor add any indirection
/// # let b_borrow_2 = b_borrow_1;
/// # let b1 = B::clone(&b_borrow_1); // but we can still clone
/// # let b2 = B::clone(&b_borrow_2); // but we can still clone
/// #
/// # let d = D(Cell::new(42));
/// # let d_borrow = Valrow::new(&d); // sadly, this compiles... for now
/// ```
///
/// Do not implement this on types where reference address identity is important.
/// ```
/// # fn addr_eq<L,R>(l: *const L, r: *const R) -> bool { (l as *const ()) == (r as *const ()) }
/// # #[cfg(introduced_after_msrv)]
/// use core::ptr::addr_eq;
///
/// use sealed::*;
/// mod sealed {
///     pub struct Singleton(());
///     pub static SINGLETON : Singleton = Singleton(());
/// }
///
/// // this may assert if and only if `Singleton: BorrowableByValue`
/// fn may_assert_if_singleton_is_borrowable_by_value(a: &Singleton, b: &Singleton) {
///     assert!(addr_eq(a, b));
/// }
///
/// // this will become unsound if and only if `Singleton: BorrowableByValue`
/// fn unsound_if_singleton_is_borrowable_by_value(a: &Singleton, b: &Singleton) {
///     if ! addr_eq(a, b) {
///         unsafe { core::hint::unreachable_unchecked() };
///     }
/// }
/// ```
///
/// Most realistic uses of such by-address identities are already forbidden by the ban on interior mutability,
/// but e.g. ZST Mutexes that track lock state by `HashMap<*const ZstMutex, std::sync::Mutex<()>>` could be broken by implementing this trait.
///
pub unsafe trait BorrowableByValue
    // : core::maker::Freeze // compiler internal, not exposed by even nightly: https://stdrs.dev/nightly/x86_64-pc-windows-gnu/core/marker/trait.Freeze.html
{
    type Abi : Copy;

    /// I considered adding this method "for safety" instead of allowing Valrow to spam `transmute`s.
    /// However, in testing it became clear that this is would actually be counterproductive.
    /// It tempts me to write `NonNull::from(&**self)` to implement `as_abi` for <code>[Arc]&lt;T&gt;</code>, but that narrows provenance.
    /// The correct code would be something like `unsafe { NonNull::new_unchecked(Arc::into_raw(core::ptr::read(self))) }`.
    ///
    /// Additionally, it cannot take a sane default impl that would discourage writing such incorrect code:
    /// ```text
    /// error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
    /// ```
    ///
    #[cfg(xxx)] fn as_abi(&self) -> Self::Abi { unsafe { *core::mem::transmute::<&Self, &Self::Abi>(self) } }
}

// TODO: make a `#[derive(BorrowableByValueZst)]` that verifies the type is a ZST?
// TODO: make a `#[derive(BorrowableByValue)]` that verifies all members are `BorrowableByValue`?
unsafe impl BorrowableByValue for () { type Abi = (); }
// TODO: add many more core/alloc/std types to improve the usability of said derive?

#[cfg(feature = "alloc")] const _ : () = {
    use core::ptr::NonNull;
    unsafe impl<T> BorrowableByValue for alloc::boxed ::Box<T> { type Abi = NonNull<T>; }
    unsafe impl<T> BorrowableByValue for alloc::rc    ::Rc <T> { type Abi = NonNull<T>; }
    unsafe impl<T> BorrowableByValue for alloc::sync  ::Arc<T> { type Abi = NonNull<T>; }
};



/// I believe the standard library currently has no T where T: Copy and T: DirectInteriorMutability.
/// This also means you cannot implement T: Copy + DirectInteriorMutability.
/// This means it's currently safe to implement BorrowableByValue for all T: Copy.
///
/// However: I don't think anyone's made a guarantee that this will always be the case for all future versions of the Rust stdlib?
/// Additionally, if we try to make a generic definition, rustc complains `Rc` might be `Copy` in the future:
///
/// ```text
/// error[E0119]: conflicting implementations of trait `borrowable::BorrowableByValue` for type `Arc<_>`
///    ...
///    = note: upstream crates may add a new impl of trait `core::marker::Copy` for type `alloc::sync::Arc<_>` in future versions
/// ```
///
/// As such, we don't actually implement this.
///
#[cfg(xxx)] unsafe impl<T: Copy> BorrowableByValue for T { type Abi = T; }
