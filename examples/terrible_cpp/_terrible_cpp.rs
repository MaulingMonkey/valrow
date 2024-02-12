//! # Wrapping Terrible C++
//!
//! Suppose we're using the following terrible C++ library:
//! ```cpp
#![doc = include_str!("ircd.cpp")]
//! ```
//!
//! There's a lot wrong with this library:
//! *   No thread safety (e.g. calling `add_user` from multiple threads is undefined behavior.)
//! *   Not reentrant (e.g. calling `add_user` from within `per_user` results in undefined behavior via invalidated iterators.)
//! *   No context parameters for callbacks (no sane way to pass captured lambda state.)
//!
//! Even so, if we can at least assume we're the only user of this library,
//! we *can* build a fairly safe and usable abstraction in Rust by creating some singleton ZSTs and exposing all FFI through them.
//! <code>[Valrow]\[[Mut](ValrowMut)\]</code> then allows passing borrows of those singletons to context-free FFI callbacks.
//!
//! ```
#![doc = include_str!("ircd.rs")]
//! #
//! # mod cxx { // mimic ircd.cpp in raw Rust
//! #   #![allow(non_upper_case_globals)]
//! #   use abistr::*;
//! #   use std::ffi::CString;
//! #
//! #   static mut channels : Vec<CString> = Vec::new();
//! #   static mut users    : Vec<CString> = Vec::new();
//! #
//! #   #[no_mangle] extern "C" fn add_channel      (channel:       CStrPtr                 ) { unsafe { channels.push(channel.to_cstr().into()) } }
//! #   #[no_mangle] extern "C" fn add_user         (user:          CStrPtr                 ) { unsafe { users   .push(user   .to_cstr().into()) } }
//! #   #[no_mangle] extern "C" fn for_each_channel (per_channel:   extern "C" fn(CStrPtr)  ) { unsafe { for channel in channels.iter() { per_channel((&**channel).into()) } } }
//! #   #[no_mangle] extern "C" fn for_each_user    (per_user:      extern "C" fn(CStrPtr)  ) { unsafe { for user    in users   .iter() { per_user(   (&**user   ).into()) } } }
//! # }
//! ```
use crate::*;
