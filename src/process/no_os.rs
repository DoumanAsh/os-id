///Raw process id type, which is dummy on this platform
pub type RawId = u8;

#[inline]
///Returns zero id, as this platform has no concept of processes
pub fn get_raw_id() -> RawId {
    0
}
