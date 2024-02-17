use crate::*;
#[cfg(doc)] use core::cell::*;



/// An inaccessible value, used to implement ABI compatability without surfacing stale values.
///
/// Given <code>struct Struct([Cell]\<[u32]\>);</code>
/// *   [`u32`] might be used in an exclusive borrow of `Struct` (ABI compatible with cell, readable)
/// *   <code>&zwj;[Opaque]\<[u32]\></code> might be used in a shared borrow of `Struct` (ABI compatible with cell, prevents reading stale contents)
///
/// However, the following conversions won't be implemented:
/// *   <code>\&[Valrow]\<Struct\> → \&Struct</code> will be impossible (it would add mutable access to an immutable field.)
/// *   <code>&zwj;[Valrow]\<Struct\> → Struct</code> would be ill advised (it would read possibly stale data.)
///
#[derive(Clone, Copy, Default)] #[repr(transparent)] pub struct Opaque<T: ?Sized = ()>(T);

impl<T> Opaque<T> {
    pub const fn new(value: T) -> Self { Self(value) }
}

// XXX: are any of these fns actually useful?
impl<T: ?Sized> Opaque<T> {
    pub const fn from_ref(reference: &    T) -> &    Self { unsafe { core::mem::transmute(reference) } }
    pub       fn from_mut(reference: &mut T) -> &mut Self { unsafe { core::mem::transmute(reference) } }
}

impl<    T        > From<        T> for         Opaque<T> { fn from(value:             T) -> Self { Self::new(value) } }
impl<'a, T: ?Sized> From<&'a     T> for &'a     Opaque<T> { fn from(reference: &'a     T) -> Self { Opaque::from_ref(reference) } }
impl<'a, T: ?Sized> From<&'a mut T> for &'a mut Opaque<T> { fn from(reference: &'a mut T) -> Self { Opaque::from_mut(reference) } }

unsafe impl<T: ?Sized + Copy> Borrowable for Opaque<T> { type Abi = Self; }

unsafe impl<    T> TransmutableFrom<        T> for         Opaque<T> {}
unsafe impl<'a, T> TransmutableFrom<&'a     T> for &'a     Opaque<T> {}
unsafe impl<'a, T> TransmutableFrom<&'a mut T> for &'a mut Opaque<T> {}
