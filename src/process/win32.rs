///Raw process id type, which is simple `u32`
pub type RawId = u32;

///Access id using `GetCurrentProcessId`
pub fn get_raw_id() -> RawId {
    extern "system" {
        pub fn GetCurrentProcessId() -> RawId;
    }

    unsafe {
        GetCurrentProcessId()
    }
}
