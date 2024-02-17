use core::num::*;
#[cfg(doc)] use core::ops::*;



/// <code>copyable!\([u8], [u16], ...\)</code> &mdash; implements the appropriate traits for types that should be value-borrowed by [`Copy`].
///
/// Do not use on types with direct interior mutability... if any ever exist.
/// While no undefined behavior should result, the results will be suprising and counterintuitive.
///
/// ### Examples
/// ```
/// # use valrow::*;
/// // As used by the crate itself:
/// # #[cfg(xxx)] {
/// copyable!(());
/// copyable!(bool, char, f32, f64);
/// copyable!(i8, i16, i32, i64, i128, isize);
/// copyable!(u8, u16, u32, u64, u128, usize);
/// copyable!(NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize);
/// copyable!(NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize);
/// # }
/// ```
///
#[macro_export] macro_rules! copyable { ( $($ty:ty),* $(,)? ) => { const _ : () = {
    #[allow(unused_imports)] use $crate::_valrow_macros_prelude::*;
    $(
        static_assert::copyable::<$ty>();
        unsafe impl valrow::Borrowable for $ty { type Abi = $ty; }
        impl valrow::IntoExclusiveBorrow for &'_ mut $ty { type Exclusive = $ty; fn into(self) -> $ty { *self } }
        impl valrow::IntoSharedBorrow    for &'_     $ty { type Shared    = $ty; fn into(self) -> $ty { *self } }
    )*
};}}

copyable!(());
copyable!(bool, char, f32, f64);
copyable!(i8, i16, i32, i64, i128, isize);
copyable!(u8, u16, u32, u64, u128, usize);
copyable!(NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize);
copyable!(NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize);



/// <code>structures! { mod IWhatever (*) { pub field: u32 } }</code> &mdash; Implement value-borrowable `struct`s
///
/// ### Examples
/// ```
/// # use valrow::*;
/// valrow::structures! {
///     // generates ISomeComInterface::{Value, Ref, Mut}
///     pub mod ISomeComInterface (*) {
///         ptr: core::ptr::NonNull<core::ffi::c_void>,
///     }
///
///     // generates RefAndValueAbiIncompatible::{Value, Ref, Mut}
///     mod RefAndValueAbiIncompatible (*) {
///         pub(super) misc_abi: Vec<u32>, // Vec layout unspecified
///     }
/// }
/// ```
///
#[macro_export] macro_rules! structures {
    () => {};

    // TODO:
    //  - support #[derive(...)]s
    //  - support doc comments

    // individual structs
    ( $(#[repr $repr:tt])? $struct_vis:vis struct &mut $struct:ident { $( $field_vis:vis $field_name:ident : $field_ty:ty ),* $(,)? } $($rest:tt)* ) => { #[derive(           )] $(#[repr $repr])? $struct_vis struct $struct<'a> { $( $field_vis $field_name: $crate::field::Exclusive<'a, $field_ty>, )* } $crate::structures! { $($rest)* } };
    ( $(#[repr $repr:tt])? $struct_vis:vis struct &    $struct:ident { $( $field_vis:vis $field_name:ident : $field_ty:ty ),* $(,)? } $($rest:tt)* ) => { #[derive(Clone, Copy)] $(#[repr $repr])? $struct_vis struct $struct<'a> { $( $field_vis $field_name: $crate::field::Shared   <'a, $field_ty>, )* } $crate::structures! { $($rest)* } };
    ( $(#[repr $repr:tt])? $struct_vis:vis struct      $struct:ident { $( $field_vis:vis $field_name:ident : $field_ty:ty ),* $(,)? } $($rest:tt)* ) => { #[derive(           )] $(#[repr $repr])? $struct_vis struct $struct     { $( $field_vis $field_name:                              $field_ty,  )* } $crate::structures! { $($rest)* } };

    // mod-bundled bulk struct impls
    ( $(#[repr $repr:tt])? $mod_vis:vis mod $mod:ident ( * ) { $( $field_vis:vis $field_name:ident : $field_ty:ty ),* $(,)? } $($rest:tt)* ) => {
        $crate::structures! {
            $(#[repr $repr])? $mod_vis mod $mod ( &mut Mut, &Ref, Value ) { $($field_vis $field_name : $field_ty),* }
            $($rest)*
        }
    };

    ( $(#[repr $repr:tt])? $mod_vis:vis mod $mod:ident ( &mut $mut:ident, & $ref:ident, $val:ident ) { $( $field_vis:vis $field_name:ident : $field_ty:ty ),* $(,)? } $($rest:tt)* ) => {
        #[allow(non_snake_case)] $mod_vis mod $mod {
            use super::*;
            $crate::structures! {
                $(#[repr $repr])? pub struct &mut $mut { $( $field_vis $field_name : $field_ty ),* }
                $(#[repr $repr])? pub struct &    $ref { $( $field_vis $field_name : $field_ty ),* }
                $(#[repr $repr])? pub struct      $val { $( $field_vis $field_name : $field_ty ),* }
            }

            const _ : () = {
                use $crate::_valrow_macros_prelude::*;

                // XXX: (A, B): AreSameType checks below are overly strict. Relax them to something like A: TransmuteInto<B>?

                impl<'a> core::ops::Deref for $mut<'a> where $(($crate::field::Exclusive<'a, $field_ty>, $crate::field::Shared<'a, $field_ty>) : AreSameType),* {
                    type Target = $ref<'a>;
                    fn deref(&self) -> &Self::Target { unsafe { core::mem::transmute(self) } }
                }

                impl<'a> core::ops::Deref for $ref<'a> where $(($crate::field::Shared<'a, $field_ty>, $field_ty) : AreSameType),* {
                    type Target = $val;
                    fn deref(&self) -> &Self::Target { unsafe { core::mem::transmute(self) } }
                }
            };
        }
        $crate::structures! { $($rest)* }
    };
}

#[doc(hidden)] pub mod _valrow_macros_prelude {
    pub use crate as valrow;
    pub use ::core;
    pub mod static_assert {
        pub const fn copyable<T: Copy>() {}
    }

    pub trait AreSameType : sealed::AreSameType {}
    impl<T> AreSameType for (T, T) {}
    mod sealed {
        pub trait AreSameType {}
        impl<T> AreSameType for (T, T) {}
    }
}
