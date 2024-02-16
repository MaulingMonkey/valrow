<center>

# 🦀 **valrow** 🦀

By-**val**ue bor**row**s

<div style="display: inline-block; text-align: left">

<!-- git -->
[![GitHub](https://img.shields.io/github/stars/MaulingMonkey/valrow.svg?label=GitHub&style=social)](https://github.com/MaulingMonkey/valrow)
[![Build Status](https://github.com/MaulingMonkey/valrow/workflows/Rust/badge.svg)](https://github.com/MaulingMonkey/valrow/actions?query=workflow%3Arust)
![Last Commit](https://img.shields.io/github/last-commit/MaulingMonkey/valrow)
<br> <!-- crates.io -->
[![crates.io](https://img.shields.io/crates/v/valrow.svg)](https://crates.io/crates/valrow)
[![docs.rs](https://img.shields.io/docsrs/valrow)](https://docs.rs/valrow)
![msrv](https://img.shields.io/crates/msrv/valrow)
<br> <!-- other -->
[![Changelog](https://img.shields.io/badge/wiki-changelog-blue?logo=github)](https://github.com/MaulingMonkey/valrow/wiki/Changelog)
[![License](https://img.shields.io/crates/l/valrow.svg)](https://github.com/MaulingMonkey/valrow)

</div></center>



## Raison d'être

Borrow smart pointers and other (small) aggregates, without <code>[Copy]</code>ing or adding indirection via references.  Use cases:
*   FFI, where extra indirection might be incompatible with fn or type signatures.
*   Borrowing singleton [ZST]s *as* [ZST]s, allowing for `FnMut → fn` adapters.
*   Micro-optimization... in theory.



## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.



## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.



<!-- references -->

[Copy]:         https://doc.rust-lang.org/core/marker/trait.Copy.html
[ZST]:          https://doc.rust-lang.org/nomicon/exotic-sizes.html#zero-sized-types-zsts
