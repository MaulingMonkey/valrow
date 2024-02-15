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
    use $crate::_valrow_macros_prelude::*;
    $(
        static_assert::copyable::<$ty>();
        unsafe impl valrow::Borrowable for $ty { type Abi = $ty; }
    )*
};}}

copyable!(());
copyable!(bool, char, f32, f64);
copyable!(i8, i16, i32, i64, i128, isize);
copyable!(u8, u16, u32, u64, u128, usize);
copyable!(NonZeroI8, NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI128, NonZeroIsize);
copyable!(NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU128, NonZeroUsize);



#[doc(hidden)] pub mod _valrow_macros_prelude {
    pub use crate as valrow;
    pub use ::core;
    pub mod static_assert {
        pub const fn copyable<T: Copy>() {}
    }
}
