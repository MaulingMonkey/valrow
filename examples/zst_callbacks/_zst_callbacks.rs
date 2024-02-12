//! ## Example: Capture uncopyable ZSTs by ZST
//!
//! ```
//! use valrow::*;
//!
//! #[derive(Debug)] pub struct ZST(()); // N.B.: not copyable for whatever reason!
//! unsafe impl BorrowableByValue for ZST { type Abi = (); } // ✔️ sound
//!
//! let zst         = ZST(());
//! let zst_ref     = &zst;
//! let zst_borrow  = Valrow::new(&zst);
//!
//! # #[cfg(wont_compile)] // rejected by StaticAssert::<Callback>::IS_ZST
//! call_me_back_asap(move || { dbg!(zst_ref);    }); // ❌ won't compile: `zst_ref` isn't a ZST
//! call_me_back_asap(move || { dbg!(zst_borrow); }); // ✔️ will compile: `zst_borrow` *is* a ZST
//!
//! fn call_me_back_asap<Callback: Fn()>(callback: Callback) {
//!     core::mem::forget(callback); // so adapt can assume a `Callback` is available
//!     return unsafe { call_me_back_asap(adapt::<Callback>) };
//!
//! #   #[cfg(nope)] {
//!     #[link(name = "clibrary")] extern "C" { fn call_me_back_asap(callback: extern "C" fn()); }
//! #   }
//! #   unsafe fn call_me_back_asap(callback: extern "C" fn()) { callback() }
//!     extern "C" fn adapt<Callback: Fn()>() {
//!         // we leaked a `Callback` earlier, so assume we can make a reference to it:
//!         let _ = StaticAssert::<Callback>::IS_ZST; // otherwise we need a real address
//!         let callback = unsafe { core::ptr::NonNull::<Callback>::dangling().as_ref() };
//!         callback();
//!     }
//! }
//! struct StaticAssert<T>(T);
//! impl<T> StaticAssert<T> { const IS_ZST : () = assert!(0 == core::mem::size_of::<T>()); }
//! ```
