//! Process id module

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

impl core::cmp::PartialEq<ProcessId> for ProcessId {
    #[inline]
    fn eq(&self, other: &ProcessId) -> bool {
        self.id == other.id
    }
}

impl core::cmp::Eq for ProcessId {}

impl core::hash::Hash for ProcessId {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl core::fmt::Display for ProcessId {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.id, f)
    }
}

impl core::fmt::LowerHex for ProcessId {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::LowerHex::fmt(&self.id, f)
    }
}

impl core::fmt::UpperHex for ProcessId {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::UpperHex::fmt(&self.id, f)
    }
}

impl core::fmt::Octal for ProcessId {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Octal::fmt(&self.id, f)
    }
}
