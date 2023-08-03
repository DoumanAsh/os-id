//! Primitive abstractions over OS identifiers.

#![no_std]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![warn(missing_docs)]

mod data;
pub mod thread;
pub mod process;

pub use data::ThreadName;
pub use thread::ThreadId;
pub use process::ProcessId;
