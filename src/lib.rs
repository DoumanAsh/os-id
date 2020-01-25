//! Contains structs to represent process and thread identifiers

#![no_std]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![warn(missing_docs)]

pub mod thread;
pub mod process;

pub use thread::ThreadId;
pub use process::ProcessId;
