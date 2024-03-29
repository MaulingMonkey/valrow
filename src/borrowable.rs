use core::ptr::NonNull;
#[cfg(doc)] use core::alloc::Layout;



/// A type that is borrowable by value.
///
/// ### Safety
/// By implementing this trait, you assert that it's safe and sound for a single instance of `Self` to exist at multiple addresses simultaneously.
/// This might exclude [`Box`, `Vec`, other users of `Unique`](<https://github.com/rust-lang/unsafe-code-guidelines/issues/326>), and types directly containing those types by value.
/// Additionally, `Self` and <code>Self::[Abi](Self::Abi)</code> must be ABI compatible:
/// *   Ideally, `#[repr(transparent)]`-compatible, so by-value borrows can substitute for owned values at FFI boundaries.
/// *   At absolute minimum, `#[repr(C)]`-compatible (same [`Layout`], same niches, same mutability, same lifetimes, ...)
///
/// Do not implement this on types with direct interior mutability.
/// The copies of the "single" instance will decohere.
/// Also, <code>[Abi](Self::Abi): [Copy]</code>, but the standard library currently provides no [`Copy`]able types that also implement interior mutability.
/// This means *you* can't implement a [`Copy`]able type that implements interior mutability.
/// This means implementing the trait would violate your promise of `Self` and `Abi` being ABI compatible.
///
/// ```rust
/// type ABox<T> = ialloc::boxed::ABox<T, ialloc::allocator::alloc::Global>;
/// # use valrow::*;
/// # use std::cell::*;
/// # use std::mem::*;
/// # use std::ptr::*;
/// # use std::rc::*;
/// # use std::sync::*;
///
/// #[repr(transparent)] struct B(Box<    usize >); // ⚠️    aliasing violation for Unique<usize>?
/// #[repr(transparent)] struct AB(ABox<  usize >); // ✔️ no aliasing violation - not Unique
/// #[repr(transparent)] struct A(Arc<    usize >); // ✔️ indirect interior mutability (refcounts)
/// #[repr(transparent)] struct R(Rc<Cell<usize>>); // ✔️ indirect interior mutability (+ Cell)
/// #[repr(transparent)] struct C(   Cell<usize> ); // ❌   direct interior mutability
/// #[repr(transparent)] struct U(        usize  ); // ✔️       no interior mutability
/// #[repr(transparent)] struct Z(          ()   ); // ✔️       no interior mutability
///
/// type Abi<T> = <T as Borrowable>::Abi;
/// unsafe impl valrow::Borrowable for B  { type Abi = NonNull<    usize  >; } // ⚠️ unsound?
/// unsafe impl valrow::Borrowable for AB { type Abi = NonNull<    usize  >; } // ✔️ sound
/// # #[cfg(feature = "alloc")] {
/// unsafe impl valrow::Borrowable for A  { type Abi = Abi<Arc<    usize >>; } // ✔️ sound
/// unsafe impl valrow::Borrowable for R  { type Abi = Abi<Rc<Cell<usize>>>; } // ✔️ sound
/// # } // cfg(feature = "alloc")
/// unsafe impl valrow::Borrowable for C  { type Abi =             usize   ; } // ❌ unsound
/// unsafe impl valrow::Borrowable for U  { type Abi =             usize   ; } // ✔️ sound
/// unsafe impl valrow::Borrowable for Z  { type Abi = Abi<          ()   >; } // ✔️ sound
///
/// // We don't have a means of correctly specifying the Copy-able Abi layout of these types
/// type AVec<T> = ialloc::vec::AVec<T, ialloc::allocator::alloc::Global>;
/// #[repr(C)] pub struct V(Vec<       usize >); // ⚠️    aliasing violation for Unique<usize>?
/// #[repr(C)] pub struct AV(AVec<     usize >); // ✔️ no aliasing violation - doesn't use Unique
/// unsafe impl valrow::Borrowable for V  { type Abi = (usize, NonNull<[usize]>); } // ❌ bad Abi
/// unsafe impl valrow::Borrowable for AV { type Abi = (usize, NonNull<[usize]>); } // ❌ bad Abi
/// #
/// # #[cfg(feature = "alloc")] {
/// # impl Clone for A { fn clone(&self) -> Self { Self(self.0.clone()) } }
/// # let a = A(Arc::new(42));
/// # let a_borrow_1 = Valrow::new(&a); // doesn't Clone A nor add any indirection
/// # let a_borrow_2 = a_borrow_1;
/// # let a1 = A::clone(&a_borrow_1); // but we can still clone
/// # let a2 = A::clone(&a_borrow_2); // but we can still clone
/// #
/// # let cell = C(Cell::new(42));
/// # let cell_borrow = Valrow::new(&cell); // sadly, this compiles... for now
/// # }
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
/// // this may fail if and only if `Singleton: Borrowable`
/// fn may_fail_if_singleton_is_borrowable_by_value(a: &Singleton, b: &Singleton) {
///     assert!(addr_eq(a, b));
/// }
///
/// // this will become unsound if and only if `Singleton: Borrowable`
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
pub unsafe trait Borrowable
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

// TODO: make a `#[derive(BorrowableZst)]` that verifies the type is a ZST?
// TODO: make a `#[derive(Borrowable)]` that verifies all members are `Borrowable`?
// TODO: add many more core/alloc/std types to improve the usability of said derive?

#[cfg(xxx)] // ❌: ABI should be more permissive for exclusive borrows
unsafe impl<'a, T: ?Sized> Borrowable for &'a mut T { type Abi = NonNull<T>;    }
unsafe impl<'a, T: ?Sized> Borrowable for &'a     T { type Abi = &'a T;         }
unsafe impl<T: ?Sized> Borrowable for *const T      { type Abi = *const T;      }
unsafe impl<T: ?Sized> Borrowable for *mut   T      { type Abi = *mut   T;      }
unsafe impl<T: ?Sized> Borrowable for NonNull<T>    { type Abi = NonNull<T>;    }

#[cfg(feature = "alloc")] const _ : () = {
    #[cfg(xxx_borrowable_box)]
    unsafe impl<T: ?Sized> Borrowable for alloc::boxed ::Box <T> { type Abi = NonNull<T>; } // ❌ UB? See try_to_break_box_valrows below.
    unsafe impl<T: ?Sized> Borrowable for alloc::rc    ::Rc  <T> { type Abi = NonNull<T>; }
    unsafe impl<T: ?Sized> Borrowable for alloc::rc    ::Weak<T> { type Abi = NonNull<T>; }
    unsafe impl<T: ?Sized> Borrowable for alloc::sync  ::Arc <T> { type Abi = NonNull<T>; }
    unsafe impl<T: ?Sized> Borrowable for alloc::sync  ::Weak<T> { type Abi = NonNull<T>; }
};



/// I believe the standard library currently has no T where T: Copy and T: DirectInteriorMutability.
/// This also means you cannot implement T: Copy + DirectInteriorMutability.
/// This means it's currently safe to implement Borrowable for all T: Copy.
///
/// However: I don't think anyone's made a guarantee that this will always be the case for all future versions of the Rust stdlib?
/// Additionally, if we try to make a generic definition, rustc complains `Rc` might be `Copy` in the future:
///
/// ```text
/// error[E0119]: conflicting implementations of trait `borrowable::Borrowable` for type `Arc<_>`
///    ...
///    = note: upstream crates may add a new impl of trait `core::marker::Copy` for type `alloc::sync::Arc<_>` in future versions
/// ```
///
/// As such, we don't actually implement this.
///
#[cfg(xxx)] unsafe impl<T: Copy> Borrowable for T { type Abi = T; }



/// [Concerns were raised](https://discord.com/channels/273534239310479360/592856094527848449/1207053316807204964)
/// about Box's Unique / noalias requirements.  Miri looks like it
/// [should be able to catch such bugs](https://github.com/rust-lang/rust/pull/94421#issuecomment-1113992481),
/// but I haven't been able to convince my copy to catch any bugs in this test.
///
/// ### Testing
/// ```cmd
/// rustup toolchain install nightly -c miri
///
/// set RUSTFLAGS=--cfg xxx_borrowable_box
/// cargo +nightly miri test --all-features
///
/// set MIRIFLAGS=-Zmiri-unique-is-unique -Zmiri-tree-borrows
/// cargo +nightly miri test --all-features
/// ```
///
/// ### References
/// *   <https://stdrs.dev/nightly/x86_64-unknown-linux-gnu/core/ptr/unique/struct.Unique.html>
/// *   <https://github.com/rust-lang/unsafe-code-guidelines/issues/326>
/// *   <https://github.com/rust-lang/miri/>
#[cfg(xxx_borrowable_box)]
#[cfg(feature = "alloc")] #[test] fn try_to_break_box_valrows() {
    let a = alloc::boxed::Box::new(core::cell::Cell::new(42));
    let b = crate::Valrow::new(&a);
    let c = &a;
    a.set(1);
    b.set(2); // possibly a problem?
    c.set(3);
    a.set(4);
    b.set(5); // possibly a problem?
    c.set(6);
    let fmt = alloc::format!("{:?}", (&a, b, c));
    #[cfg(feature = "std")] std::println!("{fmt}");

    // Maybe the temp-Deref s aren't a problem, but would having a persistent pair of different-address `&Box<Cell<_>>`s trigger miri?
    let b : &alloc::boxed::Box<_> = &*b;
    a.set( 7);
    b.set( 8); // possibly a problem?
    c.set( 9);
    a.set(10);
    b.set(11); // possibly a problem?
    c.set(12);
    let fmt = alloc::format!("{:?}", (&a, b, c));
    #[cfg(feature = "std")] std::println!("{fmt}");
}
