///Raw process id type, which is opaque type, platform dependent
pub type RawId = libc::pid_t;

#[inline]
///Access id using `getpid`
pub fn get_raw_id() -> RawId {
    unsafe {
        libc::getpid()
    }
}

