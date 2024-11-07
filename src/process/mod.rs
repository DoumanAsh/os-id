//! Process id module

use core::{cmp, hash, fmt};

#[cfg(windows)]
mod win32;
#[cfg(windows)]
pub use win32::*;
#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::*;
#[cfg(all(not(unix), not(windows)))]
mod no_os;
#[cfg(all(not(unix), not(windows)))]
pub use no_os::*;

#[repr(transparent)]
#[derive(Copy, Clone, Debug)]
///Process identifier.
pub struct ProcessId {
    id: RawId,
}

impl ProcessId {
    #[inline]
    ///Gets current process id
    pub fn current() -> Self {
        Self {
            id: get_raw_id(),
        }
    }

    #[inline]
    ///Access Raw identifier.
    pub const fn as_raw(&self) -> RawId {
        self.id
    }
}

impl cmp::PartialEq<ProcessId> for ProcessId {
    #[inline]
    fn eq(&self, other: &ProcessId) -> bool {
        self.id == other.id
    }
}

impl cmp::Eq for ProcessId {}

impl hash::Hash for ProcessId {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl fmt::Display for ProcessId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.id, f)
    }
}

impl fmt::LowerHex for ProcessId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.id, f)
    }
}

impl fmt::UpperHex for ProcessId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.id, f)
    }
}

impl fmt::Octal for ProcessId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Octal::fmt(&self.id, f)
    }
}
