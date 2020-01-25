//! Thread id module

#[cfg(windows)]
///Raw thread id type, which is simple `u32`
pub type RawId = u32;

#[cfg(unix)]
///Raw thread id type, which is opaque type, platform dependent
pub type RawId = libc::pthread_t;

#[cfg(all(not(unix), not(windows)))]
///Raw thread id type, which is dummy on this platform
pub type RawId = u8;

#[cfg(windows)]
#[inline]
///Access id using `GetCurrentThreadId`
pub fn get_raw_id() -> RawId {
    extern "system" {
        pub fn GetCurrentThreadId() -> RawId;
    }

    unsafe {
        GetCurrentThreadId()
    }
}

#[cfg(unix)]
#[inline]
///Access id using `pthread_self`
pub fn get_raw_id() -> RawId {
    unsafe {
        libc::pthread_self()
    }
}

#[cfg(all(not(unix), not(windows)))]
#[inline]
///Returns zero id, as this platform has no concept of threads
pub fn get_raw_id() -> RawId {
    0
}

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

impl core::cmp::PartialEq<ThreadId> for ThreadId {
    #[cfg(not(unix))]
    fn eq(&self, other: &ThreadId) -> bool {
        self.id == other.id
    }

    #[cfg(unix)]
    fn eq(&self, other: &ThreadId) -> bool {
        extern "system" {
            pub fn pthread_equal(left: RawId, right: RawId) -> libc::c_int;
        }

        unsafe {
            pthread_equal(self.id, other.id) != 0
        }
    }
}

impl core::hash::Hash for ThreadId {
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
