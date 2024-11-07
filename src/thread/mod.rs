//! Thread id module

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
///Thread identifier.
pub struct ThreadId {
    id: RawId,
}

impl ThreadId {
    #[inline]
    ///Gets current thread id
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

impl cmp::Eq for ThreadId {}

impl cmp::PartialEq<ThreadId> for ThreadId {
    #[inline(always)]
    fn eq(&self, other: &ThreadId) -> bool {
        raw_thread_eq(self.id, other.id)
    }
}

impl hash::Hash for ThreadId {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl fmt::Display for ThreadId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.id, f)
    }
}

impl fmt::LowerHex for ThreadId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::LowerHex::fmt(&self.id, f)
    }
}

impl fmt::UpperHex for ThreadId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::UpperHex::fmt(&self.id, f)
    }
}

impl fmt::Octal for ThreadId {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Octal::fmt(&self.id, f)
    }
}
