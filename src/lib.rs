//! Primitive abstractions over OS identifiers.
//!
//!## Features
//!
//!- `process` - Enables process module. Default ON.
//!- `thread` - Enables thread module. Default ON.

#![no_std]
#![allow(clippy::style)]
#![warn(missing_docs)]

#[cfg(feature = "thread")]
mod data;
#[cfg(feature = "thread")]
pub mod thread;
#[cfg(feature = "process")]
pub mod process;

#[cfg(feature = "thread")]
pub use data::ThreadName;
#[cfg(feature = "thread")]
pub use thread::ThreadId;
#[cfg(feature = "process")]
pub use process::ProcessId;
