//! Process id module

#[cfg(windows)]
///Raw process id type, which is simple `u32`
pub type RawId = u32;

#[cfg(unix)]
///Raw process id type, which is opaque type, platform dependent
pub type RawId = libc::pid_t;

#[cfg(all(not(unix), not(windows)))]
///Raw process id type, which is dummy on this platform
pub type RawId = u8;

#[cfg(windows)]
#[inline]
///Access id using `GetCurrentProcessId`
pub fn get_raw_id() -> RawId {
    extern "system" {
        pub fn GetCurrentProcessId() -> RawId;
    }

    unsafe {
        GetCurrentProcessId()
    }
}

#[cfg(unix)]
#[inline]
///Access id using `getpid`
pub fn get_raw_id() -> RawId {
    unsafe {
        libc::getpid()
    }
}

#[cfg(all(not(unix), not(windows)))]
#[inline]
///Returns zero id, as this platform has no concept of processes
pub fn get_raw_id() -> RawId {
    0
}

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
    fn eq(&self, other: &ProcessId) -> bool {
        self.id == other.id
    }
}

impl core::hash::Hash for ProcessId {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
