use crate::*;

#[cfg(all(doc, feature = "alloc"))] use alloc::boxed::Box;
#[cfg(all(doc, feature = "alloc"))] use alloc::vec::Vec;

use core::cmp::Ordering;
use core::fmt::{self, Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::mem::{align_of, size_of, transmute};
use core::ops::{Deref, DerefMut};



/// A by-value mutable/exclusive borrow.  Only usable for Zero Sized Types.  Only useful if <code>\![Copy]</code>.
///
/// A "by-value exclusive borrow" effectively has:
/// *   *shared* (by-value) access to *direct* data (e.g. the ptr/length/capacity/allocator of a [`Vec`])
/// *   *exclusive* access to *indirect* data (e.g. the *elements* of a [`Vec`])
///
/// No simple reference type can represent this mixed-exclusivity model.
///
/// [`ValrowMut`] only manages to support ZSTs because exclusive access to nothing is equivalent to shared access to nothing.
///
///
///
/// ### Alternatives
/// *   Own a (frozen) instance         &mdash; e.g. `move`, [`Copy`], [`Clone`], fetch atomic loads, etc.
/// *   Discard access to direct data   &mdash; e.g. reborrow <code>[Box]\<T\></code> → `&mut T`, discarding access to ownership and allocator)
/// *   Discard mutable access          &mdash; e.g. reborrow <code>[Arc]\<T\></code> → <code>[Valrow]\<[Arc]\<T\>\></code>
/// *   Write your own custom type.
///     While macros could *theoretically* help with trivial examples of this, in practice it seems anything useful would need to be largely hand-written.
///     The following example will use <code>[ABox]</code> as if we were writing `ialloc`:
///
/// ```rust
/// use ialloc::boxed::ABox;
/// use ialloc::traits::fat::Free;
///
/// use valrow::Valrow;
///
/// use core::marker::PhantomData;
/// use core::mem::ManuallyDrop;
/// use core::ops::{Deref, DerefMut};
///
/// # #[cfg(nope)]
/// unsafe impl<T, A: Free> valrow::Borrowable for ABox<T, A> {}
///
/// #[repr(transparent)] pub struct ABoxMut<'b, T, A: Free>(
///     ManuallyDrop<       ABox<T, A>>,
///     PhantomData<&'b mut ABox<T, A>>,
/// );
///
/// impl<'b, T, A: Free> ABoxMut<'b, T, A> {
///     pub fn new(reference: &'b mut ABox<T, A>) -> Self {
///         Self(ManuallyDrop::new(unsafe { core::ptr::read(reference) }), PhantomData)
///     }
///
///     # #[cfg(nope)] // can't impl Borrowable for foreign crate
///     pub fn borrow(this: &    Self) -> Valrow<ABox<T, A>> { Valrow::new(&this.0) }
///     pub fn as_ref(this: &    Self) -> &    T { &    *this.0 }
///     pub fn as_mut(this: &mut Self) -> &mut T { &mut *this.0 } // sound
///     pub fn allocator(this: & Self) -> &A     { ABox::allocator(&this.0) }
///     // NOTE: `&Self` → `&mut A` would be unsound, so don't expose that
/// }
///
/// # #[cfg(nope)]
/// impl<'b, T, A: Free> Deref    for ABoxMut<'b, T, A> { type Target = T; /* ... */ }
/// # #[cfg(nope)]
/// impl<'b, T, A: Free> DerefMut for ABoxMut<'b, T, A> { /* ... */ }
/// ```
///
///
/// ### What would actually explode with `ValrowMut` on !ZSTs?
///
/// Pretty much everything.
///
/// ```rust
/// # #[cfg(nope)] {
/// let mut value   = Box::new(1);
/// let mut borrow  = ValrowMut::new(&mut value);
/// **borrow = 1; // double indirection is awkward, but okay, reasonable
/// core::mem::replace(&mut *borrow, Box::new(2));  // frees Box(1)
/// drop(borrow);   // leaks Box(2)
/// drop(value);    // frees Box(1) (double free / undefined behavior!)
///
/// let mut value = vec![1];
/// let mut borrow = ValrowMut::new(&mut value);
/// borrow.push(2);             // might resize the buffer
/// drop(borrow);               // leaks new allocation
/// assert!(value.len() == 2);  // fails - `size` wasn't updated
/// drop(value);                // double free if `push` reallocated
///
/// let mut value = vec![1, 2, 3];
/// let mut borrow = ValrowMut::new(&mut value);
/// borrow.clear();             // drops elements
/// drop(borrow);
/// assert!(value.len() == 3);  // we have access to 3 dropped elements
///
/// let mut value = Cell::new(1);
/// let mut borrow = ValrowMut::new(&mut value);
/// borrow.set(2);
/// assert_eq!(2, value.get()); // fails - copy discarded
/// # }
/// ```
///
///
///
/// [ABox]: https://docs.rs/ialloc/0.0.0-2023-05-28/ialloc/boxed/struct.ABox.html
///
#[repr(transparent)] pub struct ValrowMut<'a, T: Borrowable>(T::Abi, PhantomData<&'a mut T>);

unsafe impl<'a, T: Borrowable   > Send              for ValrowMut<'a, T> where &'a mut T : Send {}
unsafe impl<'a, T: Borrowable   > Sync              for ValrowMut<'a, T> where &'a mut T : Sync {}
impl<    T: Borrowable          > AsRef<T>          for ValrowMut<'_, T> { fn as_ref(&    self)                   -> &    T           { Self::as_ref(self) }                              }
impl<    T: Borrowable          > AsMut<T>          for ValrowMut<'_, T> { fn as_mut(&mut self)                   -> &mut T           { Self::as_mut(self) }                              }
impl<    T: Borrowable          > Deref             for ValrowMut<'_, T> { fn deref    (&    self)                -> &    T           { Self::as_ref(self) } type Target = T;             }
impl<    T: Borrowable          > DerefMut          for ValrowMut<'_, T> { fn deref_mut(&mut self)                -> &mut T           { Self::as_mut(self) }                              }
impl<'a, T: Borrowable          > From<&'a mut T>   for ValrowMut<'a, T> { fn from(reference: &'a mut T) -> Self { Self::new(reference) } }

// XXX: actually, how many of these traits are really needed?
impl<T: Borrowable + Debug      > Debug             for ValrowMut<'_, T> { fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result      { <T as Debug       >::fmt(self, fmt) }             }
impl<T: Borrowable + Display    > Display           for ValrowMut<'_, T> { fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result      { <T as Display     >::fmt(self, fmt) }             }
impl<T: Borrowable + PartialEq  > PartialEq         for ValrowMut<'_, T> { fn eq(&self, other: &Self)             -> bool             { <T as PartialEq   >::eq(self, other) }            }
impl<T: Borrowable + Eq         > Eq                for ValrowMut<'_, T> {}
impl<T: Borrowable + PartialOrd > PartialOrd        for ValrowMut<'_, T> { fn partial_cmp(&self, other: &Self)    -> Option<Ordering> { <T as PartialOrd  >::partial_cmp(self, other) }   }
impl<T: Borrowable + Ord        > Ord               for ValrowMut<'_, T> { fn cmp(&self, other: &Self)            -> Ordering         { <T as Ord         >::cmp(self, other) }           }
impl<T: Borrowable + Hash       > Hash              for ValrowMut<'_, T> { fn hash<H: Hasher>(&self, state: &mut H)                   { <T as Hash        >::hash(self, state) }          }

impl<'a, T: Borrowable> ValrowMut<'a, T> {
    /// Borrow `*reference` by value.
    #[inline(always)] pub fn new(reference: &'a mut T) -> Self {
        let _ = Self::STATIC_CHECK_T_ABI;
        Self(unsafe { *transmute::<&mut T, &mut T::Abi>(reference) }, PhantomData)
    }

    fn as_ref(&self) -> &T {
        let _ = Self::STATIC_CHECK_T_ABI;
        unsafe { transmute(self) }
    }

    fn as_mut(&mut self) -> &mut T {
        let _ = Self::STATIC_CHECK_T_ABI;
        unsafe { transmute(self) }
    }

    /// Makes an effort to validate `T` and `T::Abi` are compatible.  This should have no false negatives?
    const STATIC_CHECK_T_ABI : () = {
        assert!(0               ==  size_of::<T     >()); // T      *must* be a ZST
        assert!(0               ==  size_of::<T::Abi>()); // T::Abi *must* be a ZST
        assert!(align_of::<T>() == align_of::<T::Abi>());
    };
}
