///Raw process id type, which is opaque type, platform dependent
pub type RawId = i32;

#[inline]
///Access id using `getpid`
pub fn get_raw_id() -> RawId {
    extern "C" {
        fn getpid() -> RawId;
    }

    unsafe {
        getpid()
    }
}

