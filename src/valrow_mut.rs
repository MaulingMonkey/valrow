use crate::*;

use core::cmp::Ordering;
use core::fmt::{self, Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};



/// A by-value mutable/exclusive borrow.  Only usable for Zero Sized Types.  Only useful if <code>\![Copy]</code>.
pub struct ValrowMut<'a, T: BorrowableByValue>(T::Abi, PhantomData<&'a mut T>);

unsafe impl<'a, T: BorrowableByValue    > Send          for ValrowMut<'a, T> where &'a mut T : Send {}
unsafe impl<'a, T: BorrowableByValue    > Sync          for ValrowMut<'a, T> where &'a mut T : Sync {}
impl<T: BorrowableByValue               > AsRef<T>      for ValrowMut<'_, T> { fn as_ref(&    self)                   -> &    T           { Self::as_ref(self) }                              }
impl<T: BorrowableByValue               > AsMut<T>      for ValrowMut<'_, T> { fn as_mut(&mut self)                   -> &mut T           { Self::as_mut(self) }                              }
impl<T: BorrowableByValue               > Deref         for ValrowMut<'_, T> { fn deref    (&    self)                -> &    T           { Self::as_ref(self) } type Target = T;             }
impl<T: BorrowableByValue               > DerefMut      for ValrowMut<'_, T> { fn deref_mut(&mut self)                -> &mut T           { Self::as_mut(self) }                              }

// XXX: actually, how many of these traits are really needed?
impl<T: BorrowableByValue + Debug       > Debug         for ValrowMut<'_, T> { fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result      { <T as Debug       >::fmt(self, fmt) }             }
impl<T: BorrowableByValue + Display     > Display       for ValrowMut<'_, T> { fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result      { <T as Display     >::fmt(self, fmt) }             }
impl<T: BorrowableByValue + PartialEq   > PartialEq     for ValrowMut<'_, T> { fn eq(&self, other: &Self)             -> bool             { <T as PartialEq   >::eq(self, other) }            }
impl<T: BorrowableByValue + Eq          > Eq            for ValrowMut<'_, T> {}
impl<T: BorrowableByValue + PartialOrd  > PartialOrd    for ValrowMut<'_, T> { fn partial_cmp(&self, other: &Self)    -> Option<Ordering> { <T as PartialOrd  >::partial_cmp(self, other) }   }
impl<T: BorrowableByValue + Ord         > Ord           for ValrowMut<'_, T> { fn cmp(&self, other: &Self)            -> Ordering         { <T as Ord         >::cmp(self, other) }           }
impl<T: BorrowableByValue + Hash        > Hash          for ValrowMut<'_, T> { fn hash<H: Hasher>(&self, state: &mut H)                   { <T as Hash        >::hash(self, state) }          }

impl<'a, T: BorrowableByValue> ValrowMut<'a, T> {
    /// Borrow `*reference` by value.
    #[inline(always)] pub fn new(reference: &'a mut T) -> Self {
        let _ = Self::STATIC_CHECK_T_ABI;
        Self(unsafe { *core::mem::transmute::<&mut T, &mut T::Abi>(reference) }, PhantomData)
    }

    fn as_ref(&self) -> &T {
        let _ = Self::STATIC_CHECK_T_ABI;
        unsafe { core::mem::transmute(self) }
    }

    fn as_mut(&mut self) -> &mut T {
        let _ = Self::STATIC_CHECK_T_ABI;
        unsafe { core::mem::transmute(self) }
    }

    /// Makes an effort to validate `T` and `T::Abi` are compatible.  This should have no false negatives?
    const STATIC_CHECK_T_ABI : () = {
        use core::mem::*;
        assert!(0               ==  size_of::<T     >()); // T      *must* be a ZST
        assert!(0               ==  size_of::<T::Abi>()); // T::Abi *must* be a ZST
        assert!(align_of::<T>() == align_of::<T::Abi>());
    };
}
