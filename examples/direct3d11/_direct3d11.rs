//! # Direct3D 11
//!
//! Consider <code>[ID3D11DeviceContext](https://learn.microsoft.com/en-us/windows/win32/api/d3d11/nn-d3d11-id3d11devicecontext)::[PSSetShaderResources](https://learn.microsoft.com/en-us/windows/win32/api/d3d11/nf-d3d11-id3d11devicecontext-pssetshaderresources)</code>.
//! Like many Direct3D11 APIs, it takes an array of pointers to COM interfaces, but does not consume your co-ownership of them.
//!
//! The [`windows` equivalent](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Direct3D11/struct.ID3D11DeviceContext.html#method.PSSetShaderResources),
//! however, forces you to create an [`array`]/[`slice`]/[`Vec`] of *owning* pointers &mdash; presumably filled with temporary [`Clone`]s, forcing you to increment their refcounts,
//! which are then [`Drop`]ped shortly after when your array is no longer necessary.
//! This is a bunch of pointless refcounting churn!
//!
//! An alternative would be for [`ID3D11ShaderResourceView`](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Direct3D11/struct.ID3D11ShaderResourceView.html)
//! to implement <code>[valrow]::[Borrowable]</code> and
//! <code>[PSSetShaderResources](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Direct3D11/struct.ID3D11DeviceContext.html#method.PSSetShaderResources)</code>
//! changed to accept <code>&amp;&zwj;\[[Option]&lt;[Valrow]&lt;[ID3D11ShaderResourceView](https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Direct3D11/struct.ID3D11ShaderResourceView.html)&gt;&gt;]</code> instead.
use crate::{self as valrow, *};
use alloc::vec::Vec;
