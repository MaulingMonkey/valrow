//! **N.B. breaks ABI:** adapt direct interior mutability fields of an aggregate type to "by value borrows".
//!
use crate::*;

#[cfg(feature = "std")] use std::sync::{Mutex, RwLock, atomic::*};

use core::cell::*;
use core::fmt::{self, Debug, Display, Formatter};
use core::mem::transmute;
use core::ops::{Deref, DerefMut};



// XXX: Make these traits instead of structs if we switch to a proc macro?

/// [`Copy`] <code>cell.[get](Cell::get)()</code>s, <code>*refcell.[borrow](RefCell::borrow)()</code>, or <code>atomic.[load](core::sync::atomic::AtomicU8::load)(...)</code> for "by value" borrows.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct ByCopy<T>(pub T);

/// [`Clone`] <code>cell.[get](Cell::get)()</code>s, <code>*refcell.[borrow](RefCell::borrow)()</code>, or <code>atomic.[load](core::sync::atomic::AtomicU8::load)(...)</code> for "by value" borrows.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct ByClone<T>(pub T);

/// Discard <code>\&<span style="opacity: 50%">mut</span> T</code> in favor of <code>[Opaque]\<()\></code> for "by value" borrows.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct ByDiscard<T>(pub T);

/// Use in directly mutable aggregate fields to indicate "by value" borrows should be <code>[Opaque]\<T\></code>.
///
/// This is similar to [`ByDiscard`], but attempts to preserve the type layout at least.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct ByOpaque<T>(pub T);

/// Make "by value" borrows a misnomer: take a reference like in regular borrowing.
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)] #[repr(transparent)] pub struct ByRef<T>(pub T);



impl<T: AsMut<O>, O>    AsMut<O>    for ByCopy      <T> { fn as_mut(&mut self) -> &mut O { self.0.as_mut() } }
impl<T: AsMut<O>, O>    AsMut<O>    for ByClone     <T> { fn as_mut(&mut self) -> &mut O { self.0.as_mut() } }
impl<T: AsMut<O>, O>    AsMut<O>    for ByDiscard   <T> { fn as_mut(&mut self) -> &mut O { self.0.as_mut() } }
impl<T: AsMut<O>, O>    AsMut<O>    for ByOpaque    <T> { fn as_mut(&mut self) -> &mut O { self.0.as_mut() } }
impl<T: AsMut<O>, O>    AsMut<O>    for ByRef       <T> { fn as_mut(&mut self) -> &mut O { self.0.as_mut() } }

impl<T: AsRef<O>, O>    AsRef<O>    for ByCopy      <T> { fn as_ref(&self) -> &O { self.0.as_ref() } }
impl<T: AsRef<O>, O>    AsRef<O>    for ByClone     <T> { fn as_ref(&self) -> &O { self.0.as_ref() } }
impl<T: AsRef<O>, O>    AsRef<O>    for ByDiscard   <T> { fn as_ref(&self) -> &O { self.0.as_ref() } }
impl<T: AsRef<O>, O>    AsRef<O>    for ByOpaque    <T> { fn as_ref(&self) -> &O { self.0.as_ref() } }
impl<T: AsRef<O>, O>    AsRef<O>    for ByRef       <T> { fn as_ref(&self) -> &O { self.0.as_ref() } }

impl<T: Debug>          Debug       for ByCopy      <T> { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Debug::fmt(&self.0, f) } }
impl<T: Debug>          Debug       for ByClone     <T> { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Debug::fmt(&self.0, f) } }
impl<T: Debug>          Debug       for ByDiscard   <T> { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Debug::fmt(&self.0, f) } }
impl<T: Debug>          Debug       for ByOpaque    <T> { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Debug::fmt(&self.0, f) } }
impl<T: Debug>          Debug       for ByRef       <T> { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Debug::fmt(&self.0, f) } }

impl<T: Display>        Display     for ByCopy      <T> { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Display::fmt(&self.0, f) } }
impl<T: Display>        Display     for ByClone     <T> { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Display::fmt(&self.0, f) } }
impl<T: Display>        Display     for ByDiscard   <T> { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Display::fmt(&self.0, f) } }
impl<T: Display>        Display     for ByOpaque    <T> { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Display::fmt(&self.0, f) } }
impl<T: Display>        Display     for ByRef       <T> { fn fmt(&self, f: &mut Formatter) -> fmt::Result { Display::fmt(&self.0, f) } }

impl<T>                 Deref       for ByCopy      <T> { fn deref(&self) -> &T { &self.0 } type Target = T; }
impl<T>                 Deref       for ByClone     <T> { fn deref(&self) -> &T { &self.0 } type Target = T; }
impl<T>                 Deref       for ByDiscard   <T> { fn deref(&self) -> &T { &self.0 } type Target = T; }
impl<T>                 Deref       for ByOpaque    <T> { fn deref(&self) -> &T { &self.0 } type Target = T; }
impl<T>                 Deref       for ByRef       <T> { fn deref(&self) -> &T { &self.0 } type Target = T; }

impl<T>                 DerefMut    for ByCopy      <T> { fn deref_mut(&mut self) -> &mut T { &mut self.0 } }
impl<T>                 DerefMut    for ByClone     <T> { fn deref_mut(&mut self) -> &mut T { &mut self.0 } }
impl<T>                 DerefMut    for ByDiscard   <T> { fn deref_mut(&mut self) -> &mut T { &mut self.0 } }
impl<T>                 DerefMut    for ByOpaque    <T> { fn deref_mut(&mut self) -> &mut T { &mut self.0 } }
impl<T>                 DerefMut    for ByRef       <T> { fn deref_mut(&mut self) -> &mut T { &mut self.0 } }

impl<T>                 From<T>     for ByCopy      <T> { fn from(value: T) -> Self { Self(value) } }
impl<T>                 From<T>     for ByClone     <T> { fn from(value: T) -> Self { Self(value) } }
impl<T>                 From<T>     for ByDiscard   <T> { fn from(value: T) -> Self { Self(value) } }
impl<T>                 From<T>     for ByOpaque    <T> { fn from(value: T) -> Self { Self(value) } }
impl<T>                 From<T>     for ByRef       <T> { fn from(value: T) -> Self { Self(value) } }



// ByCopy
impl<'a, T> IntoExclusiveBorrow for &'a mut ByCopy<T> where &'a mut ByClone<T> : IntoExclusiveBorrow { type Exclusive = <&'a mut ByClone<T> as IntoExclusiveBorrow>::Exclusive; fn into(self) -> Self::Exclusive { IntoExclusiveBorrow::into(unsafe { transmute::<&mut ByCopy<T>, &mut ByClone<T>>(self) }) } }
impl<'a, T> IntoSharedBorrow    for &'a     ByCopy<T> where &'a     ByClone<T> : IntoSharedBorrow    { type Shared    = <&'a     ByClone<T> as IntoSharedBorrow   >::Shared   ; fn into(self) -> Self::Shared    { IntoSharedBorrow   ::into(unsafe { transmute::<&    ByCopy<T>, &    ByClone<T>>(self) }) } }

// ByClone
#[cfg(feature = "core")] impl<T: Copy> IntoExclusiveBorrow for &'_ mut ByClone <   Cell<T>> { type Exclusive = T ; fn into(self) -> Self::Exclusive { T::clone(&self.0.get()) } }
#[cfg(feature = "core")] impl<T: Copy> IntoExclusiveBorrow for &'_ mut ByClone <RefCell<T>> { type Exclusive = T ; fn into(self) -> Self::Exclusive { T::clone(&*self.0.borrow()) } }
#[cfg(feature = "std" )] impl<T: Copy> IntoExclusiveBorrow for &'_ mut ByClone <  Mutex<T>> { type Exclusive = T ; fn into(self) -> Self::Exclusive { T::clone(&*self.0.lock().unwrap()) } }
#[cfg(feature = "std" )] impl<T: Copy> IntoExclusiveBorrow for &'_ mut ByClone < RwLock<T>> { type Exclusive = T ; fn into(self) -> Self::Exclusive { T::clone(&*self.0.read().unwrap()) } }
#[cfg(feature = "core")] impl<T: Copy> IntoSharedBorrow    for &'_     ByClone <   Cell<T>> { type Shared    = T ; fn into(self) -> Self::Shared    { T::clone(&self.0.get()) } }
#[cfg(feature = "core")] impl<T: Copy> IntoSharedBorrow    for &'_     ByClone <RefCell<T>> { type Shared    = T ; fn into(self) -> Self::Shared    { T::clone(&*self.0.borrow()) } }
#[cfg(feature = "std" )] impl<T: Copy> IntoSharedBorrow    for &'_     ByClone <  Mutex<T>> { type Shared    = T ; fn into(self) -> Self::Shared    { T::clone(&*self.0.lock().unwrap()) } }
#[cfg(feature = "std" )] impl<T: Copy> IntoSharedBorrow    for &'_     ByClone < RwLock<T>> { type Shared    = T ; fn into(self) -> Self::Shared    { T::clone(&*self.0.read().unwrap()) } }

// ByDiscard
impl<    T> IntoExclusiveBorrow for &'_ mut ByDiscard<T> { type Exclusive = Opaque; fn into(self) -> Opaque { Opaque::new(()) } }
impl<    T> IntoSharedBorrow    for &'_     ByDiscard<T> { type Shared    = Opaque; fn into(self) -> Opaque { Opaque::new(()) } }

// ByRef
impl<'a, T> IntoExclusiveBorrow for &'a mut ByRef<T> { type Exclusive = &'a mut T; fn into(self) -> Self::Exclusive { &mut self.0 } }
impl<'a, T> IntoSharedBorrow    for &'a     ByRef<T> { type Shared    = &'a     T; fn into(self) -> Self::Shared    { &    self.0 } }

// ByOpaque
impl<T: Copy> IntoExclusiveBorrow for &'_ mut ByOpaque<Cell<T>> { type Exclusive = Opaque<T>; fn into(self) -> Self::Exclusive { Opaque::new(self.get()) } }
impl<T: Copy> IntoSharedBorrow    for &'_     ByOpaque<Cell<T>> { type Shared    = Opaque<T>; fn into(self) -> Self::Shared    { Opaque::new(self.get()) } }



#[cfg(feature = "core")] const _ : () = {
    macro_rules! atomic { ( $($atomic:ident($int:ident) ),* $(,)? ) => {$(
        impl IntoExclusiveBorrow for &'_ mut ByClone <$atomic> { type Exclusive =        $int ; fn into(self) ->        $int  { self.load(Ordering::Relaxed) } }
        impl IntoSharedBorrow    for &'_     ByClone <$atomic> { type Shared    =        $int ; fn into(self) ->        $int  { self.load(Ordering::Relaxed) } }
        impl IntoExclusiveBorrow for &'_ mut ByOpaque<$atomic> { type Exclusive = Opaque<$int>; fn into(self) -> Opaque<$int> { Opaque::new(self.load(Ordering::Relaxed)) } }
        impl IntoSharedBorrow    for &'_     ByOpaque<$atomic> { type Shared    = Opaque<$int>; fn into(self) -> Opaque<$int> { Opaque::new(self.load(Ordering::Relaxed)) } }
    )*}}

    #[cfg(target_has_atomic =   "8")] atomic!(AtomicI8(i8), AtomicU8(u8), AtomicBool(bool));
    #[cfg(target_has_atomic =  "16")] atomic!(AtomicI16(i16), AtomicU16(u16));
    #[cfg(target_has_atomic =  "32")] atomic!(AtomicI32(i32), AtomicU32(u32));
    #[cfg(target_has_atomic =  "64")] atomic!(AtomicI64(i64), AtomicU64(u64));
    #[cfg(target_has_atomic = "128")] atomic!(AtomicI128(i128), AtomicU128(u128));
    #[cfg(target_has_atomic = "ptr")] atomic!(AtomicIsize(isize), AtomicUsize(usize));
    #[cfg(target_has_atomic = "ptr")] impl<T> IntoExclusiveBorrow for &'_ mut ByClone <AtomicPtr<T>> { type Exclusive = *mut T; fn into(self) -> *mut T { self.load(Ordering::Relaxed) } }
    #[cfg(target_has_atomic = "ptr")] impl<T> IntoSharedBorrow    for &'_     ByClone <AtomicPtr<T>> { type Shared    = *mut T; fn into(self) -> *mut T { self.load(Ordering::Relaxed) } }
    #[cfg(target_has_atomic = "ptr")] impl<T> IntoExclusiveBorrow for &'_ mut ByOpaque<AtomicPtr<T>> { type Exclusive = Opaque<*mut T>; fn into(self) -> Opaque<*mut T> { Opaque::new(self.load(Ordering::Relaxed)) } } // XXX: AtomicPtr is Send
    #[cfg(target_has_atomic = "ptr")] impl<T> IntoSharedBorrow    for &'_     ByOpaque<AtomicPtr<T>> { type Shared    = Opaque<*mut T>; fn into(self) -> Opaque<*mut T> { Opaque::new(self.load(Ordering::Relaxed)) } } // XXX: AtomicPtr is Send
};
