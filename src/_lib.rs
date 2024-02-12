#![no_std]
#![doc = include_str!("../Readme.md")]



#[cfg(any(doc, feature = "alloc"))] extern crate alloc;
#[cfg(any(doc, feature = "std"  ))] extern crate std;
#[cfg(doc)] use alloc::{rc::Rc, sync::Arc};



#[cfg(doc)] #[path = "../examples"] pub mod Examples {
    //! [Direct3D 11](Direct3D11), [Wrapping Terrible C++](TerribleCxx), [ZST Callbacks](ZstCallbacks)

    #[path = "direct3d11/_direct3d11.rs"] pub mod Direct3D11;
    #[path = "terrible_cpp/_terrible_cpp.rs"] pub mod TerribleCxx;
    #[path = "zst_callbacks/_zst_callbacks.rs"] pub mod ZstCallbacks;
}

mod borrowable;     pub use crate::borrowable::*;
mod valrow;         pub use crate::valrow::*;
mod valrow_mut;     pub use crate::valrow_mut::*;
