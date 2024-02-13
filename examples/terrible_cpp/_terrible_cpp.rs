//! # Wrapping Terrible C++
//!
//! Suppose we're using the following terrible C++ library:
//! ```cpp
#![doc = include_str!("ircd.cpp")]
//! ```
//!
//! There's a lot wrong with this library.  We'll ignore:
//! *   Vague string encoding (ASCII? UTF8? Windows-1251? System Locale? Which one?)
//! *   Potential for [static initialization order fiasco](https://en.cppreference.com/w/cpp/language/siof).
//! *   [`std::bad_alloc` exceptions](https://en.cppreference.com/w/cpp/memory/new/bad_alloc) could cross FFI boundaries.
//! *   Probably more I haven't bothered to list.
//!
//! However, if we can at least assume we're the only user of this library,
//! <code>[Valrow]\[[Mut](ValrowMut)\]</code> and ZSTs can help us tackle *these* issues:
//! *   Not reentrant (e.g. calling `add_user` from within `per_user` results in undefined behavior via invalidated iterators.)<br>
//!     Fixed by guarding global state with singleton ZSTs.<br>
//!     <br>
//! *   No thread safety (e.g. calling `add_user` from multiple threads is undefined behavior.)<br>
//!     Fixed by putting said singleton ZSTs within rust-owned [`Mutex`](std::sync::Mutex)es.<br>
//!     <br>
//! *   No context parameters for callbacks (no sane way to pass captured lambda state.)<br>
//!     Fixed by static asserting `FnMut()`s are ZSTs, which can still capture borrows via <code>[Valrow]\[[Mut](ValrowMut)\]</code>.
//!     <br>
//!
//! # Demonstration
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
