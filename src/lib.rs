#![forbid(unsafe_code)]
#![deny(missing_debug_implementations, nonstandard_style, rust_2021_compatibility)]
#![warn(missing_docs, rustdoc::missing_doc_code_examples, unreachable_pub)]

//! getter is a package for downloading files or directories from a variety of
//! protocols.
//!
//! getter is unique in its ability to download both directories and files.
//! It also detects certain source strings to be protocol-specific URLs. For
//! example, "github.com/blessanabraham/rs-getter" would turn into a Git URL and
//! use the Git protocol.
//!
//! Protocols and detectors are extensible.
//!
//! To get started, see Client.

mod error;
pub use error::*;

mod client_mode;
pub use crate::client_mode::ClientMode;

pub mod detector;
pub use crate::detector::{detect, DETECTORS};

mod client;

mod getter;
pub use crate::getter::GETTERS;
