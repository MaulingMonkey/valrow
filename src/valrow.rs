use crate::*;

use core::cmp::Ordering;
use core::fmt::{self, Debug, Display, Formatter};
use core::hash::{Hash, Hasher};
use core::marker::PhantomData;
use core::ops::Deref;



/// A by-value borrow.
pub struct Valrow<'a, T: Borrowable>(T::Abi, PhantomData<&'a T>);

unsafe impl<'a, T: Borrowable   > Send          for Valrow<'a, T> where &'a T : Send {}
unsafe impl<'a, T: Borrowable   > Sync          for Valrow<'a, T> where &'a T : Sync {}
impl<T: Borrowable              > Copy          for Valrow<'_, T> {}
impl<T: Borrowable              > Clone         for Valrow<'_, T> { fn clone(&self)                        -> Self             { Self(self.0, self.1) }                            }
impl<T: Borrowable              > AsRef<T>      for Valrow<'_, T> { fn as_ref(&self)                       -> &T               { Self::as_ref(self) }                              }
impl<T: Borrowable              > Deref         for Valrow<'_, T> { fn deref(&self)                        -> &T               { Self::as_ref(self) } type Target = T;             }

// XXX: actually, how many of these traits are really needed?
impl<T: Borrowable + Debug      > Debug         for Valrow<'_, T> { fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result      { <T as Debug       >::fmt(self, fmt) }             }
impl<T: Borrowable + Display    > Display       for Valrow<'_, T> { fn fmt(&self, fmt: &mut Formatter<'_>) -> fmt::Result      { <T as Display     >::fmt(self, fmt) }             }
impl<T: Borrowable + PartialEq  > PartialEq     for Valrow<'_, T> { fn eq(&self, other: &Self)             -> bool             { <T as PartialEq   >::eq(self, other) }            }
impl<T: Borrowable + Eq         > Eq            for Valrow<'_, T> {}
impl<T: Borrowable + PartialOrd > PartialOrd    for Valrow<'_, T> { fn partial_cmp(&self, other: &Self)    -> Option<Ordering> { <T as PartialOrd  >::partial_cmp(self, other) }   }
impl<T: Borrowable + Ord        > Ord           for Valrow<'_, T> { fn cmp(&self, other: &Self)            -> Ordering         { <T as Ord         >::cmp(self, other) }           }
impl<T: Borrowable + Hash       > Hash          for Valrow<'_, T> { fn hash<H: Hasher>(&self, state: &mut H)                   { <T as Hash        >::hash(self, state) }          }

impl<'a, T: Borrowable> Valrow<'a, T> {
    /// Borrow `*reference` by value.
    #[inline(always)] pub const fn new(reference: &'a T) -> Self {
        let _ = Self::STATIC_CHECK_T_ABI;
        Self(unsafe { *core::mem::transmute::<&T, &T::Abi>(reference) }, PhantomData)
    }

    fn as_ref(&self) -> &T {
        let _ = Self::STATIC_CHECK_T_ABI;
        unsafe { core::mem::transmute(self) }
    }

    /// Makes an effort to validate `T` and `T::Abi` are compatible.  This may have false negatives (e.g. fail to trip despite incompatability.)
    const STATIC_CHECK_T_ABI : () = {
        use core::mem::*;
        assert!(align_of::<       T >() == align_of::<T::Abi>());
        assert!( size_of::<       T >() ==  size_of::<T::Abi>());
        assert!(align_of::<Option<T>>() == align_of::<Option<T::Abi>>()); // imperfect check for niche compatability
        assert!( size_of::<Option<T>>() ==  size_of::<Option<T::Abi>>()); // imperfect check for niche compatability
    };
}
