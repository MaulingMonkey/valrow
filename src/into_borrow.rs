use crate::*;
use core::marker::*;
use core::ptr::NonNull;



/// Convert `&mut s.field` into the corresponding field of a "borrowed by value `s`"
pub trait IntoSharedBorrow {
    type Shared : Copy;
    fn into(self) -> Self::Shared;
}

/// Convert `&    s.field` into the corresponding field of a "exclusively borrowed by value `s`"
pub trait IntoExclusiveBorrow {
    type Exclusive;
    fn into(self) -> Self::Exclusive;
}

pub mod field {
    //! <code>[Exclusive]\<\'a, T\></code>,
    //! <code>[Shared]\<\'a, T\></code>

    /// <code>\<\&\'a     T as [IntoSharedBorrow]   \>::Shared</code>
    pub type Shared<'a, T>      = <&'a     T as crate::IntoSharedBorrow>::Shared;

    /// <code>\<\&\'a mut T as [IntoExclusiveBorrow]\>::Exclusive</code>
    pub type Exclusive<'a, T>   = <&'a mut T as crate::IntoExclusiveBorrow>::Exclusive;
}

// Marker borrows

impl<T: ?Sized + IntoSharedBorrow   > IntoSharedBorrow      for &'_     PhantomData<T> { type Shared    = PhantomData<T>; fn into(self) -> Self::Shared    { PhantomData } }
impl<T: ?Sized + IntoExclusiveBorrow> IntoExclusiveBorrow   for &'_ mut PhantomData<T> { type Exclusive = PhantomData<T>; fn into(self) -> Self::Exclusive { PhantomData } }

// Pointery borrows

impl<'a, T: ?Sized> IntoExclusiveBorrow     for &'a mut &'a mut T   { type Exclusive  = &'a mut T ; fn into(self) -> Self::Exclusive    { *self } }
impl<'a, T: ?Sized> IntoExclusiveBorrow     for &'a mut &'a     T   { type Exclusive  = &'a     T ; fn into(self) -> Self::Exclusive    { *self } }
impl<    T: ?Sized> IntoExclusiveBorrow     for &'_ mut *mut    T   { type Exclusive  = *mut    T ; fn into(self) -> Self::Exclusive    { *self } }
impl<    T: ?Sized> IntoExclusiveBorrow     for &'_ mut *const  T   { type Exclusive  = *const  T ; fn into(self) -> Self::Exclusive    { *self } }
impl<    T: ?Sized> IntoExclusiveBorrow     for &'_ mut NonNull<T>  { type Exclusive  = NonNull<T>; fn into(self) -> Self::Exclusive    { *self } }

impl<'a, T: ?Sized> IntoSharedBorrow        for &'a     &'a mut T   { type Shared     = &'a     T ; fn into(self) -> Self::Shared       { *self } }
impl<'a, T: ?Sized> IntoSharedBorrow        for &'a     &'a     T   { type Shared     = &'a     T ; fn into(self) -> Self::Shared       { *self } }
impl<    T: ?Sized> IntoSharedBorrow        for &'_     *mut    T   { type Shared     = *mut    T ; fn into(self) -> Self::Shared       { *self } }
impl<    T: ?Sized> IntoSharedBorrow        for &'_     *const  T   { type Shared     = *const  T ; fn into(self) -> Self::Shared       { *self } }
impl<    T: ?Sized> IntoSharedBorrow        for &'_     NonNull<T>  { type Shared     = NonNull<T>; fn into(self) -> Self::Shared       { *self } }

#[cfg(feature = "alloc")] const _ : () = {
    use alloc::{boxed::Box, rc::{self, Rc}, sync::{Arc, Weak}, vec::Vec};

    impl<'a, T: ?Sized> IntoExclusiveBorrow for &'a mut Box<T>      { type Exclusive  = &'a mut  T;               fn into(self) -> Self::Exclusive  { &mut *self } }
    impl<'a, T: ?Sized> IntoSharedBorrow    for &'a     Box<T>      { type Shared     = &'a      T;               fn into(self) -> Self::Shared     { &    *self } }
    impl<'a, T        > IntoExclusiveBorrow for &'a mut Vec<T>      { type Exclusive  = &'a mut [T];              fn into(self) -> Self::Exclusive  { &mut **self } }
    impl<'a, T        > IntoSharedBorrow    for &'a     Vec<T>      { type Shared     = &'a     [T];              fn into(self) -> Self::Shared     { &    **self } }

    impl<'a, T: ?Sized> IntoExclusiveBorrow for &'a mut Arc<T>      { type Exclusive  = Valrow<'a, Arc<T>>;       fn into(self) -> Self::Exclusive  { Valrow::new(self) } }
    impl<'a, T: ?Sized> IntoSharedBorrow    for &'a     Arc<T>      { type Shared     = Valrow<'a, Arc<T>>;       fn into(self) -> Self::Shared     { Valrow::new(self) } }
    impl<'a, T: ?Sized> IntoExclusiveBorrow for &'a mut Weak<T>     { type Exclusive  = Valrow<'a, Weak<T>>;      fn into(self) -> Self::Exclusive  { Valrow::new(self) } }
    impl<'a, T: ?Sized> IntoSharedBorrow    for &'a     Weak<T>     { type Shared     = Valrow<'a, Weak<T>>;      fn into(self) -> Self::Shared     { Valrow::new(self) } }

    impl<'a, T: ?Sized> IntoExclusiveBorrow for &'a mut Rc<T>       { type Exclusive  = Valrow<'a, Rc<T>>;        fn into(self) -> Self::Exclusive  { Valrow::new(self) } }
    impl<'a, T: ?Sized> IntoSharedBorrow    for &'a     Rc<T>       { type Shared     = Valrow<'a, Rc<T>>;        fn into(self) -> Self::Shared     { Valrow::new(self) } }
    impl<'a, T: ?Sized> IntoExclusiveBorrow for &'a mut rc::Weak<T> { type Exclusive  = Valrow<'a, rc::Weak<T>>;  fn into(self) -> Self::Exclusive  { Valrow::new(self) } }
    impl<'a, T: ?Sized> IntoSharedBorrow    for &'a     rc::Weak<T> { type Shared     = Valrow<'a, rc::Weak<T>>;  fn into(self) -> Self::Shared     { Valrow::new(self) } }
};



// Enums.

impl<'a, T> IntoExclusiveBorrow for &'a mut Option<T> where &'a mut T : IntoExclusiveBorrow {
    type Exclusive = Option<<&'a mut T as IntoExclusiveBorrow>::Exclusive>;
    fn into(self) -> Self::Exclusive { self.as_mut().map(IntoExclusiveBorrow::into) }
}

impl<'a, T> IntoSharedBorrow for &'a Option<T> where &'a T : IntoSharedBorrow {
    type Shared = Option<<&'a T as IntoSharedBorrow>::Shared>;
    fn into(self) -> Self::Shared { self.as_ref().map(IntoSharedBorrow::into) }
}

impl<'a, O, E> IntoExclusiveBorrow for &'a mut Result<O, E> where &'a mut O : IntoExclusiveBorrow, &'a mut E : IntoExclusiveBorrow {
    type Exclusive = Result<<&'a mut O as IntoExclusiveBorrow>::Exclusive, <&'a mut E as IntoExclusiveBorrow>::Exclusive>;
    fn into(self) -> Self::Exclusive { match self.as_mut() { Ok(o) => Ok(IntoExclusiveBorrow::into(o)), Err(e) => Err(IntoExclusiveBorrow::into(e)) } }
}

impl<'a, O, E> IntoSharedBorrow for &'a Result<O, E> where &'a O : IntoSharedBorrow, &'a E : IntoSharedBorrow {
    type Shared = Result<<&'a O as IntoSharedBorrow>::Shared, <&'a E as IntoSharedBorrow>::Shared>;
    fn into(self) -> Self::Shared { match self.as_ref() { Ok(o) => Ok(IntoSharedBorrow::into(o)), Err(e) => Err(IntoSharedBorrow::into(e)) } }
}
