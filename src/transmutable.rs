// Related documentation / crates:
//  *   https://doc.rust-lang.org/core/intrinsics/fn.transmute.html
//  *   https://doc.rust-lang.org/nomicon/transmutes.html
//  *   https://rust-lang.github.io/rfcs/2835-project-safe-transmute.html
//  *   https://docs.rs/safe-transmute/
//  *   https://docs.rs/bytemuck/



/// Marker trait indicating <code>[core]::[mem](core::mem)::[transmute](core::mem::transmute)::&lt;Self, O&gt;(...)</code> is safe to call.
pub unsafe trait TransmutableFrom<O> {}

/// T can always be transmuted into itself
unsafe impl<T> TransmutableFrom<T> for T {}

/// An exclusive reference to T can be reborrowed as shared references to T
unsafe impl<'a, T> TransmutableFrom<&'a mut T> for &'a T {}



/// Marker trait indicating <code>[core]::[mem](core::mem)::[transmute](core::mem::transmute)::&lt;O, Self&gt;(...)</code> is safe to call.
pub unsafe trait TransmutableInto<O> {}

unsafe impl<T, O> TransmutableInto<O> for T where O : TransmutableFrom<T> {}
