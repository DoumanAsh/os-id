//! Primitive abstractions over OS identifiers.
//!
//! Features
//!
//! - `thread-name` Enables function `get_current_thread_name`

#![no_std]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::style))]
#![warn(missing_docs)]

pub mod thread;
pub mod process;

pub use thread::ThreadId;
pub use process::ProcessId;
