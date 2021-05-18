#[cfg(all(unix, not(target_os = "linux"), not(target_os = "android"), not(target_os = "macos"), not(target_os = "ios"), not(target_os = "netbsd"), not(target_os = "freebsd")))]
///Raw thread id type, which is opaque type, platform dependent
pub type RawId = libc::pthread_t;

#[cfg(any(target_os = "linux", target_os = "android"))]
///Raw thread id as `pid_t` which is signed integer
///
///Can be accessed via `gettid` on Linux and Android
pub type RawId = libc::pid_t;

#[cfg(target_os = "freebsd")]
///Raw thread id signed integer
///
///Can be accessed via `pthread_threadid_np` on freebsd
pub type RawId = libc::c_int;

#[cfg(target_os = "netbsd")]
///Raw thread id unsigned integer
///
///Can be accessed via `_lwp_self` on netbsd
pub type RawId = libc::c_uint;

#[cfg(any(target_os = "macos", target_os = "ios"))]
///Raw thread id as unsigned 64 bit integer.
///
///Can be accessed via `pthread_threadid_np` on mac
pub type RawId = u64;

#[cfg(target_os = "freebsd")]
#[inline]
///Accesses id using `pthread_threadid_np`
pub fn get_raw_id() -> RawId {
    #[link(name = "pthread")]
    extern "C" {
        fn pthread_getthreadid_np() -> libc::c_int;
    }

    //According to documentation it cannot fail
    unsafe { pthread_getthreadid_np() }
}

#[cfg(target_os = "netbsd")]
#[inline]
///Accesses id using `_lwp_self`
pub fn get_raw_id() -> RawId {
    extern "C" {
        fn _lwp_self() -> libc::c_uint;
    }

    //According to documentation it cannot fail
    unsafe { _lwp_self() }
}

#[cfg(any(target_os = "macos", target_os = "ios"))]
#[inline]
///Accesses id using `pthread_threadid_np`
pub fn get_raw_id() -> RawId {
    #[link(name = "pthread")]
    extern "C" {
        fn pthread_threadid_np(thread: libc::pthread_t, thread_id: *mut u64) -> libc::c_int;
    }
    let mut tid: u64 = 0;
    let err = unsafe { pthread_threadid_np(0, &mut tid) };
    assert_eq!(err, 0);
    tid
}

#[cfg(any(target_os = "android", target_os = "linux"))]
#[inline]
///Accesses id using `gettid`
pub fn get_raw_id() -> RawId {
    unsafe { libc::syscall(libc::SYS_gettid) as libc::pid_t }
}

#[cfg(all(unix, not(target_os = "linux"), not(target_os = "android"), not(target_os = "macos"), not(target_os = "ios"), not(target_os = "netbsd"), not(target_os = "freebsd")))]
#[inline]
///Access id using `pthread_self`
pub fn get_raw_id() -> RawId {
    unsafe {
        libc::pthread_self()
    }
}

#[inline(always)]
///Thread equality function.
///
///Guarantees to compare regardless of raw type.
pub fn raw_thread_eq(left: RawId, right: RawId) -> bool {
    #[cfg(any(target_os = "linux", target_os = "android", target_os = "macos", target_os = "ios", target_os = "netbsd", target_os = "freebsd"))]
    {
        left == right
    }

    #[cfg(all(not(target_os = "linux"), not(target_os = "android"), not(target_os = "macos"), not(target_os = "ios"), not(target_os = "netbsd"), not(target_os = "freebsd")))]
    {
        #[link(name = "pthread")]
        extern "C" {
            pub fn pthread_equal(left: RawId, right: RawId) -> libc::c_int;
        }

        unsafe {
            pthread_equal(left, right) != 0
        }
    }
}

#[cfg(feature = "thread-name")]
///Accesses current thread name using `pthread_getname_np`.
pub fn get_current_thread_name() -> str_buf::StrBuf::<16> {
    extern "C" {
        pub fn pthread_getname_np(thread: libc::pthread_t, name: *mut i8, len: libc::size_t) -> libc::c_int;
    }

    let mut storage = [0u8; 16];

    let result = unsafe {
        pthread_getname_np(libc::pthread_self(), storage.as_mut_ptr() as _, storage.len() as _)
    };

    if result == 0 {
        let slice = if let Some(null_idx) = storage.iter().position(|b| *b == b'\0') {
            &storage[..null_idx]
        } else {
            &storage[..]
        };

        match core::str::from_utf8(slice) {
            Ok(res) => return str_buf::StrBuf::from_str(res),
            _ => (),
        }
    }

    str_buf::StrBuf::new()
}
