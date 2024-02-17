#![no_std]
#![debugger_visualizer(natvis_file = "../debug/valrow.natvis")]
#![doc = include_str!("../Readme.md")]



#[cfg(any(doc, feature = "alloc"))] extern crate alloc;
#[cfg(any(doc, feature = "std"  ))] extern crate std;
#[cfg(doc)] use alloc::{rc::Rc, sync::Arc};



#[cfg(doc)] #[path = "../examples"] pub mod Examples {
    //! [Direct3D 11](Direct3D11), [Wrapping Terrible C++](TerribleCxx), [ZST Callbacks](ZstCallbacks)

    #[path = "aggregates/_aggregates.rs"] pub mod Aggregates;
    #[path = "direct3d11/_direct3d11.rs"] pub mod Direct3D11;
    #[path = "terrible_cpp/_terrible_cpp.rs"] pub mod TerribleCxx;
    #[path = "zst_callbacks/_zst_callbacks.rs"] pub mod ZstCallbacks;
}

#[macro_use] mod macros; #[doc(hidden)] pub use macros::_valrow_macros_prelude;
pub mod adapt;
mod borrowable;     pub use crate::borrowable::*;
mod into_borrow;    pub use into_borrow::*;
mod opaque;         pub use crate::opaque::*;
mod transmutable;   pub use transmutable::*;
mod valrow;         pub use crate::valrow::*;
mod valrow_mut;     pub use crate::valrow_mut::*;
