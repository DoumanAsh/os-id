///Raw thread id type, which is simple `u32`
pub type RawId = u32;

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

#[inline(always)]
///Thread equality function.
///
///Guarantees to compare regardless of raw type.
pub fn raw_thread_eq(left: RawId, right: RawId) -> bool {
    left == right
}

#[cfg(feature = "thread-name")]
///Accesses thread name using `GetThreadDescription`.
///
///Only compiles on Win10.
pub fn get_current_thread_name() -> str_buf::StrBuf::<16> {
    use core::ptr;
    use winapi::shared::winerror::HRESULT;
    use winapi::um::winnt::HANDLE;
    use winapi::um::winnt::PWSTR;
    use winapi::um::processthreadsapi::GetCurrentThread;
    use winapi::um::winnls::CP_UTF8;
    use winapi::um::stringapiset::WideCharToMultiByte;

    extern "system" {
        pub fn GetThreadDescription(handle: HANDLE, out: *mut PWSTR) -> HRESULT;
    }

    let mut result = str_buf::StrBuf::new();
    let mut desc_ptr = core::ptr::null_mut();

    let code = unsafe {
        GetThreadDescription(GetCurrentThread(), &mut desc_ptr)
    };

    if code < 0 {
        return result;
    }

    let storage = result.as_write_slice();
    unsafe {
        let len = WideCharToMultiByte(CP_UTF8, 0, desc_ptr as _, -1, storage.as_mut_ptr() as _, storage.len() as _, ptr::null(), ptr::null_mut()) as _;
        result.set_len(len);
        if let Some(null_idx) = result.as_slice().iter().position(|b| *b == b'\0') {
            result.set_len(null_idx as _);
        }
        winapi::um::winbase::LocalFree(desc_ptr as _);
    };

    result
}
