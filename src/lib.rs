//! An allocator-aware, `no_std`-compatible implementation of `String<A>` that mirrors `std::string::String`.
//!
//! This crate provides a custom string implementation that supports custom allocators while maintaining
//! full compatibility with the standard library's string functionality.
//!
//! ## Requirements
//!
//! This crate requires the nightly toolchain due to its use of the `allocator_api` feature.
//! Add the following to your `rust-toolchain.toml` or use `cargo +nightly`:
//!
//! ```toml
//! [toolchain]
//! channel = "nightly"
//! ```
//!
//! ## Features
//!
//! - UTF-8 correctness
//! - Full `no_std` support via `extern crate alloc`
//! - Custom allocator compatibility
//! - Thread-safe operations
//! - `format_in!` macro support
//! - Serde serialization/deserialization (optional)
//!
//! ## Design Choices
//!
//! This implementation closely mirrors the standard library's `String` type while making some
//! deliberate choices to keep the codebase small and safe:
//!
//! - **Allocator Support**: All methods that allocate take an explicit allocator parameter with the `_in` suffix
//!   to distinguish them from the default allocator versions.
//!
//! - **UTF-8 Safety**: All string operations maintain UTF-8 correctness, with proper handling of
//!   character boundaries and byte lengths.
//!
//! - **Minimal Dependencies**: The implementation uses only stable features and core functionality,
//!   avoiding unstable features to maintain compatibility and safety.
//!
//! ### Omitted Features
//!
//! Some features from the standard library's `String` implementation have been intentionally omitted:
//!
//! - `from_utf8_lossy`: Requires unstable features for efficient lossy UTF-8 conversion
//! - `get`/`get_mut`: Can be worked around using string slicing and `split_at`
//! - `drain`: Can be replaced with `split_off` and `retain` for most use cases
//!
//! These omissions are intentional to:
//! - Keep the codebase small and maintainable
//! - Avoid unstable features
//! - Maintain safety guarantees
//! - Provide workable alternatives through existing methods
//!
//! ## Usage
//!
//! ```rust
//! #![feature(allocator_api)]
//!
//! use string_alloc::{String, format_in};
//! use std::alloc::Global;
//!
//! // Basic usage
//! let mut s = String::from_str_in("hello", Global);
//! s.push_str(" world");
//!
//! // Using format_in! macro
//! let name = "Alice";
//! let s2 = format_in!(Global, "Hello, {}!", name);
//!
//! // With serde (requires "serde" feature)
//! #[cfg(feature = "serde")]
//! {
//!     use serde::{Serialize, Deserialize};
//!     
//!     #[derive(Serialize, Deserialize)]
//!     struct Person {
//!         name: String<Global>,
//!     }
//! }
//! ```
//!
//! ## License
//!
//! Apache-2.0

#![no_std]
#![feature(allocator_api)]

#[cfg(feature = "std")] extern crate std;

extern crate alloc;

pub mod string;
pub use string::String;
