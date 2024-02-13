use crate::*;

use core::cmp::Ordering;
use core::fmt::{self, Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::mem::{align_of, size_of, transmute};
use core::ops::{Deref, DerefMut};



/// A by-value mutable/exclusive borrow.  Only usable for Zero Sized Types.  Only useful if <code>\![Copy]</code>.
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
