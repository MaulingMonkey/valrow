//! Traits and types for building up aggregate "by-value" borrows on a per-field basis.
//!
//! ### Borrowable by-value
//! Borrowing basic fields is straightforward:
//!
//! | field type <br> **Values**            | → | exclusive <br> "by value"                                         | shared <br> "by value"                            | `S`   | `&S`  | notes |
//! |:-------------------------------------:|:-:|:-----------------------------------------------------------------:|:-------------------------------------------------:|:-----:|:-----:|:-----:|
//! | [`u32`]                               | ✔️| <code style="opacity: 33%">[u32]</code>                           | [`u32`]                                           | ✔️ | ✔️ | transmute
//! | <code>[Range]\<[u32]\></code>         | ✔️| <code style="opacity: 33%">[Range]\<[u32]\></code>                | <code>[Range]\<[u32]\></code>                     | ✔️ | ✔️ | transmute
//! | `()`                                  | ✔️| <code style="opacity: 33%">()</code>                              | `()`                                              | ✔️ | ✔️ | transmute
//! | **Smart Pointers**
//! | <code>[Box]\<[u32], ZST\></code>      | ✔️| <code>\&mut [u32]</code>                                          | <code>\&[u32]</code>                              | ✔️ | ⚠️ | unique
//! | <code>[Box]\<[u32], !ZST\></code>     | ✔️| <code>\&mut [u32]</code>                                          | <code>\&[u32]</code>                              | ✔️ | ❌ | layout (!allocator)
//! | <code>[ABox]\<[u32], ZST\></code>     | ✔️| <code>\&mut [u32]</code>                                          | <code>\&[u32]</code>                              | ✔️ | ✔️ | transmute
//! | <code>[Arc]\<[u32]\></code>           | ✔️| <code style="opacity: 33%">[Valrow]\<[Arc]\<[u32]\>\></code>      | <code>[Valrow]\<[Arc]\<[u32]\>\></code>           | ✔️ | ✔️ | transmute
//! | <code>[Rc]\<[u32]\></code>            | ✔️| <code style="opacity: 33%">[Valrow]\<[Rc]\<[u32]\>\></code>       | <code>[Valrow]\<[Rc]\<[u32]\>\></code>            | ✔️ | ✔️ | transmute
//! | <code>[Weak]\<[u32]\></code>          | ✔️| <code style="opacity: 33%">[Valrow]\<[Weak]\<[u32]\>\></code>     | <code>[Valrow]\<[Weak]\<[u32]\>\></code>          | ✔️ | ✔️ | transmute
//! | **Containers**
//! | <code>[Box]\<\[[u32]\], ZST\></code>  | ✔️| <code>\&mut \[[u32]\]</code>                                      | <code>\&\[[u32]\]</code>                          | ✔️ | ⚠️ | unique
//! | <code>[Box]\<\[[u32]\], !ZST\></code> | ✔️| <code>\&mut \[[u32]\]</code>                                      | <code>\&\[[u32]\]</code>                          | ✔️ | ❌ | layout (!allocator)
//! | <code>[ABox]\<\[[u32]\], ZST\></code> | ✔️| <code>\&mut \[[u32]\]</code>                                      | <code>\&\[[u32]\]</code>                          | ✔️ | ✔️ | transmute
//! | <code>[Arc]\<\[[u32]\]\></code>       | ✔️| <code style="opacity: 33%">[Valrow]\<[Arc]\<\[[u32]\]\>\></code>  | <code>[Valrow]\<[Arc]\<\[[u32]\]\>\></code>       | ✔️ | ✔️ | transmute
//! | <code>[Rc]\<\[[u32]\]\></code>        | ✔️| <code style="opacity: 33%">[Valrow]\<[Rc]\<\[[u32]\]\>\></code>   | <code>[Valrow]\<[Rc]\<\[[u32]\]\>\></code>        | ✔️ | ✔️ | transmute
//! | <code>[Weak]\<\[[u32]\]\></code>      | ✔️| <code style="opacity: 33%">[Valrow]\<[Weak]\<\[[u32]\]\>\></code> | <code>[Valrow]\<[Weak]\<\[[u32]\]\>\></code>      | ✔️ | ✔️ | transmute
//! | <code>[Vec]\<[u32], ...\></code>      | ✔️| <code>\&mut [u32]</code>                                          | <code>\&[u32]</code>                              | ✔️ | ❌ | layout (!capacity)
//! | <code>[Vec]\<[u32], ...\></code>      | ✔️| <code style="opacity: 33%">[Valrow]\<[Vec]\<[u32], ...\>\></code> | <code>[Valrow]\<[Vec]\<[u32], ...\>\></code>      | ✔️ | ⚠️ | unique
//! | <code>[AVec]\<[u32], ...\></code>     | ✔️| <code>\&mut [u32]</code>                                          | <code>\&[u32]</code>                              | ✔️ | ❌ | layout (!capacity)
//! | <code>[AVec]\<[u32], ...\></code>     | ✔️| <code style="opacity: 33%">[Valrow]\<[AVec]\<[u32], ...\>\></code>| <code>[Valrow]\<[AVec]\<[u32], ...\>\></code>     | ✔️ | ✔️ | transmute
//! | **Strings** (w/o `mut` contents)
//! | <code>[ABox]\<[CStr], ...\></code>    | ✔️| <code style="opacity: 33%">[Valrow]\<...\></code>                 | <code>[Valrow]\<[ABox]\<[CStr], ...\>\></code>    | ✔️ | ✔️ | transmute
//! | <code>[ABox]\<[OsStr], ...\></code>   | ✔️| <code style="opacity: 33%">[Valrow]\<...\></code>                 | <code>[Valrow]\<[ABox]\<[OsStr], ...\>\></code>   | ✔️ | ✔️ | transmute
//! | <code>[ABox]\<[Path], ...\></code>    | ✔️| <code style="opacity: 33%">[Valrow]\<...\></code>                 | <code>[Valrow]\<[ABox]\<[Path], ...\>\></code>    | ✔️ | ✔️ | transmute
//! | <code>[ABox]\<[str], ...\></code>     | ✔️| <code style="opacity: 33%">[Valrow]\<...\></code>                 | <code>[Valrow]\<[ABox]\<[str], ...\>\></code>     | ✔️ | ✔️ | transmute
//! | <code>[Box]\<..., ...\></code>        | ✔️| <code style="opacity: 33%">[Valrow]\<...\></code>                 | <code>[Valrow]\<[Box]\<..., ...\>\></code>        | ✔️ | ⚠️ | unique
//! | [`CString`]                           | ✔️| <code style="opacity: 33%">[Valrow]\<...\></code>                 | <code>[Valrow]\<[CString]\></code>                | ✔️ | ❌ | layout (!capacity)
//! | [`OsString`]                          | ✔️| <code style="opacity: 33%">[Valrow]\<...\></code>                 | <code>[Valrow]\<[OsString]\></code>               | ✔️ | ❌ | layout (!capacity)
//! | [`PathBuf`]                           | ✔️| <code style="opacity: 33%">[Valrow]\<...\></code>                 | <code>[Valrow]\<[PathBuf]\></code>                | ✔️ | ❌ | layout (!capacity)
//! | [`String`]                            | ✔️| <code style="opacity: 33%">[Valrow]\<...\></code>                 | <code>[Valrow]\<[String]\></code>                 | ✔️ | ❌ | layout (!capacity)
//! | **Strings** (w/ `mut` contents)
//! | <code>[ABox]\<[CStr], ZST\></code>    | ✔️| <code>\&mut [CStr]</code>                                         | <code>\&[CStr]</code>                             | ✔️ | ✔️ | transmute
//! | <code>[ABox]\<[OsStr], ZST\></code>   | ✔️| <code>\&mut [OsStr]</code>                                        | <code>\&[OsStr]</code>                            | ✔️ | ✔️ | transmute
//! | <code>[ABox]\<[Path], ZST\></code>    | ✔️| <code>\&mut [Path]</code>                                         | <code>\&[Path]</code>                             | ✔️ | ✔️ | transmute
//! | <code>[ABox]\<[str], ZST\></code>     | ✔️| <code>\&mut [str]</code>                                          | <code>\&[str]</code>                              | ✔️ | ✔️ | transmute
//! | <code>[ABox]\<..., !ZST\></code>      | ✔️| <code>\&mut ...</code>                                            | <code>\&...</code>                                | ✔️ | ❌ | layout (!allocator)
//! | <code>[Box]\<..., ZST\></code>        | ✔️| <code>\&mut ...</code>                                            | <code>\&...</code>                                | ✔️ | ⚠️ | unique
//! | <code>[Box]\<..., !ZST\></code>       | ✔️| <code>\&mut ...</code>                                            | <code>\&...</code>                                | ✔️ | ❌ | layout (!allocator)
//! | [`CString`]                           | ✔️| <code>\&mut [CStr]</code>                                         | <code>\&[CStr]</code>                             | ✔️ | ❌ | layout (!capacity)
//! | [`OsString`]                          | ✔️| <code>\&mut [OsStr]</code>                                        | <code>\&[OsStr]</code>                            | ✔️ | ❌ | layout (!capacity)
//! | [`PathBuf`]                           | ✔️| <code>\&mut [Path]</code>                                         | <code>\&[Path]</code>                             | ✔️ | ❌ | layout (!capacity)
//! | [`String`]                            | ✔️| <code>\&mut [str]</code>                                          | <code>\&[str]</code>                              | ✔️ | ❌ | layout (!capacity)
//!
//! [ABox]: https://docs.rs/ialloc/0.0.0-2023-05-28/ialloc/boxed/struct.ABox.html
//! [AVec]: https://docs.rs/ialloc/0.0.0-2023-05-28/ialloc/vec/struct.AVec.html

use crate::*;

#[cfg(feature = "alloc")] use alloc::boxed::Box;
#[cfg(feature = "alloc")] use alloc::rc::{self, Rc};
#[cfg(feature = "alloc")] use alloc::sync::{Arc, Weak};
#[cfg(feature = "alloc")] use alloc::string::String;
#[cfg(feature = "alloc")] use alloc::vec::Vec;

#[cfg(feature = "std")] use std::ffi::{CStr, CString, OsStr, OsString};
#[cfg(feature = "std")] use std::path::{Path, PathBuf};

#[cfg(doc)] use core::ops::Range;






/// <code>\<\'s, F\> = \<\&\'s     F as [IntoShared]   \>::Field</code>
pub type Shared<'s, F>      = <&'s     F as IntoShared>::Field;

/// <code>\<\'s, F\> = \<\&\'s mut F as [IntoExclusive]\>::Field</code>
pub type Exclusive<'s, F>   = <&'s mut F as IntoExclusive>::Field;

/// <code>(\&\'s     s.field) → [Shared]   \<\'s, Field></code>
pub fn shared<'s,    F>(field: &'s     F) -> Shared<'s,    F> where &'s F : IntoShared { <&'s F as IntoShared>::into(field) }

/// <code>(\&\'s mut s.field) → [Exclusive]\<\'s, Field></code>
pub fn exclusive<'s, F>(field: &'s mut F) -> Exclusive<'s, F> where &'s mut F : IntoExclusive { <&'s mut F as IntoExclusive>::into(field) }

/// Implement conversion of `&mut s.field` into a borrowed "by value" `s.field`
pub trait IntoShared { fn into(self) -> Self::Field; type Field : Copy; }

/// Implement conversion of `&    s.field` into an exclusively borrowed "by value" `s.field`
pub trait IntoExclusive { fn into(self) -> Self::Field; type Field; }
