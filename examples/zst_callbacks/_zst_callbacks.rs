//! ## ZST Callbacks
//!
//! ```
//! use valrow::*;
//!
//! #[derive(Debug)] pub struct ZST(()); // N.B.: not copyable for whatever reason!
//! unsafe impl valrow::Borrowable for ZST { type Abi = (); } // ✔️ sound
//!
//! let zst         = ZST(());
//! let zst_ref     = &zst;
//! let zst_borrow  = Valrow::new(&zst);
//!
//! # #[cfg(wont_compile)] // rejected by StaticAssert::<Callback>::IS_ZST
//! call_callback(move || { dbg!(zst_ref);    }); // ❌ won't compile: `zst_ref` isn't a ZST
//! call_callback(move || { dbg!(zst_borrow); }); // ✔️ will compile: `zst_borrow` *is* a ZST
//!
//! fn call_callback<Callback: FnMut()>(mut callback: Callback) {
//!     let callback = &mut callback;
//!     unsafe { call_callback(adapt::<Callback>) };
//!     let _ = callback;
//!
//! #   #[cfg(nope)] {
//!     #[link(name = "clibrary")] extern "C" { fn call_callback(callback: extern "C" fn()); }
//! #   }
//! #   unsafe fn call_callback(callback: extern "C" fn()) { callback() }
//!     extern "C" fn adapt<Callback: FnMut()>() {
//!         // ⚠️ assume only `call_callback` calls `adapt` (has a `&mut Callback` on the stack)
//!         let _ = StaticAssert::<Callback>::IS_ZST; // otherwise we need a real address
//!         let callback = unsafe { core::ptr::NonNull::<Callback>::dangling().as_mut() };
//!         callback();
//!     }
//! }
//! struct StaticAssert<T>(T);
//! impl<T> StaticAssert<T> { const IS_ZST : () = assert!(0 == core::mem::size_of::<T>()); }
//! ```
