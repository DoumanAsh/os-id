use crate::ThreadName;

///Raw thread id type, which is dummy on this platform
pub type RawId = u8;

#[inline]
///Returns zero id, as this platform has no concept of threads
pub fn get_raw_id() -> RawId {
    0
}

#[inline(always)]
///Thread equality function.
///
///Guarantees to compare regardless of raw type.
pub fn raw_thread_eq(left: RawId, right: RawId) -> bool {
    left == right
}

#[inline(always)]
///Returns empty thread name as this target has no concept of threads.
pub fn get_current_thread_name() -> ThreadName {
    ThreadName::new()
}
