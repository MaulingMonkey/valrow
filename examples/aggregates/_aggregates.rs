//! # Aggregates
//!
//! | `struct S` field type                         | to exclusive      <br> exclusive type                 | to shared          <br> shared type                   | to `&S` <br> to `S`    |
//! | ----------------------------------------------| ------------------------------------------------------| ------------------------------------------------------| -----------------------|
//! | [`u32`]                                       | <code>[transmute]\(...\)</code>  ✔️ <br> [`u32`]                        | <code>[transmute]\(...\)</code> ✔️  <br> [`u32`]                       | ✔️ <code>[transmute]\(...\)</code>                <br> ✔️ [`Copy`]
//! |
//! | <code>ByCopy\<[Cell]\<[u32]\>\></code>        | <code>[transmute]\(...\)</code>  ✔️ <br> [`u32`]                        | <code>[transmute]\(...\)</code> ✔️  <br> [`u32`]                       | ❌ Aliases const → mut          <br> ⚠️ Stale [`Copy`]
//! | <code>ByOpaque\<[Cell]\<[u32]\>\></code>      | <code>[transmute]\(...\)</code>  ✔️ <br> <code>Opaque\<[u32]\></code>   | <code>[transmute]\(...\)</code> ✔️  <br> <code>Opaque\<[u32]\></code>  | ❌ Aliases const → mut          <br> ❌ Stale (≈ Discarded if uninit?)
//! | <code>ByRef\<[Cell]\<[u32]\>\></code>         | [`Into`]       ⚠️ <br> <code>\&mut [u32]</code>       | <code>[transmute]\(...\)</code> ✔️  <br> <code>\&[u32]</code>          | ❌ Wrong indirection level      <br> ⚠️ <code>.[get](Cell::get)()</code> if `T: Copy`
//! | <code>ByDiscard\<[Cell]\<[u32]\>\></code>     | [`Drop`]       ⚠️ <br> `Opaque`                       | <code>[transmute]\(...\)</code> ✔️  <br> `Opaque`                      | ❌ Entirely wrong layout        <br> ❌ Discarded
//! |
//! | <code>ByCopy\<[RefCell]\<[u32]\>\></code>     | <code>.[borrow](RefCell::borrow)().[clone](Clone::clone)() ⚠️ <br> [`u32`]                        | <code>[transmute]\(...\)</code> ✔️  <br> [`u32`]                       | ❌ Aliases const → mut          <br> ⚠️ Stale [`Copy`]
//! | <code>ByOpaque\<[RefCell]\<[u32]\>\></code>   | <code>.[borrow](RefCell::borrow)().[clone](Clone::clone)() ⚠️ <br> <code>Opaque\<[u32]\></code>   | <code>[transmute]\(...\)</code> ✔️  <br> <code>Opaque\<[u32]\></code>  | ❌ Aliases const → mut          <br>
//! | <code>ByRef\<[RefCell]\<[u32]\>\></code>      | [`Into`]       ⚠️ <br> <code>\&mut [u32]</code>       | <code>[transmute]\(...\)</code> ✔️  <br> <code>\&[u32]</code>          | ❌ Wrong indirection level      <br> ⚠️ <code>.[borrow](RefCell::borrow)().[clone](Clone::clone)()</code> if `T: Clone`
//! | <code>ByDiscard\<[RefCell]\<[u32]\>\></code>  | [`Drop`]       ⚠️ <br> `()`                           | <code>[transmute]\(...\)</code> ✔️  <br> `()`                          | ❌ Entirely wrong layout        <br>


use core::cell::*;
use core::ops::*;
use core::mem::*;
