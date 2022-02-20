use crate::ThreadName;

use core::ptr;

///Raw thread id type, which is simple `u32`
pub type RawId = u32;

extern "system" {
    fn GetCurrentThread() -> isize;
    fn WideCharToMultiByte(codepage: u32, dwflags: u32, lpwidecharstr: *const u16, cchwidechar: i32, lpmultibytestr: *const u8, cbmultibyte: i32, lpdefaultchar: *const u8, lpuseddefaultchar: *mut i32) -> i32;
    fn LocalFree(hmem: isize) -> isize;
    fn GetCurrentThreadId() -> RawId;
    fn GetModuleHandleA(lpmodulename: *const u8) -> isize;
    fn GetProcAddress(hmodule: isize, lpprocname: *const u8) -> isize;
}

#[inline]
///Access id using `GetCurrentThreadId`
pub fn get_raw_id() -> RawId {
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


//Reference
//https://github.com/rust-lang/rust/blob/673d0db5e393e9c64897005b470bfeb6d5aec61b/library/std/src/sys/windows/compat.rs
macro_rules! compat_fn {
    ($module:literal: $(
        $(#[$meta:meta])*
        pub fn $symbol:ident($($argname:ident: $argtype:ty),*) -> $rettype:ty $fallback_body:block
    )*) => ($(
        $(#[$meta])*
        pub mod $symbol {
            #[allow(unused_imports)]
            use super::*;
            use core::mem;

            type F = unsafe extern "system" fn($($argtype),*) -> $rettype;

            /// Points to the DLL import, or the fallback function.
            ///
            /// This static can be an ordinary, unsynchronized, mutable static because
            /// we guarantee that all of the writes finish during CRT initialization,
            /// and all of the reads occur after CRT initialization.
            static mut PTR: Option<F> = None;

            /// This symbol is what allows the CRT to find the `init` function and call it.
            /// It is marked `#[used]` because otherwise Rust would assume that it was not
            /// used, and would remove it.
            #[used]
            #[link_section = ".CRT$XCU"]
            static INIT_TABLE_ENTRY: unsafe extern "C" fn() = init;

            unsafe extern "C" fn init() {
                // There is no locking here. This code is executed before main() is entered, and
                // is guaranteed to be single-threaded.
                //
                // DO NOT do anything interesting or complicated in this function! DO NOT call
                // any Rust functions or CRT functions, if those functions touch any global state,
                // because this function runs during global initialization. For example, DO NOT
                // do any dynamic allocation, don't call LoadLibrary, etc.
                let module_name: *const u8 = concat!($module, "\0").as_ptr();
                let symbol_name: *const u8 = concat!(stringify!($symbol), "\0").as_ptr();
                let module_handle = GetModuleHandleA(module_name);
                if module_handle != 0 {
                    match GetProcAddress(module_handle, symbol_name) as usize {
                        0 => {}
                        n => {
                            PTR = Some(mem::transmute::<usize, F>(n));
                        }
                    }
                }
            }

            #[allow(dead_code)]
            pub fn option() -> Option<F> {
                unsafe { PTR }
            }

            #[allow(dead_code)]
            pub unsafe fn call($($argname: $argtype),*) -> $rettype {
                if let Some(ptr) = PTR {
                    ptr($($argname),*)
                } else {
                    $fallback_body
                }
            }
        }

        $(#[$meta])*
        pub use $symbol::call as $symbol;
    )*)
}

///Accesses thread name using `GetThreadDescription`.
///
///Only available on Win10.
pub fn get_current_thread_name() -> ThreadName {
    const CP_UTF8: u32 = 65001u32;

    compat_fn! {
        "kernel32":

        // >= Win10
        #[allow(non_snake_case)]
        pub fn GetThreadDescription(handle: isize, out: *mut *const u16) -> i32 {
            return -1
        }
    }

    let mut desc_ptr = ptr::null();

    let code = unsafe {
        GetThreadDescription(GetCurrentThread(), &mut desc_ptr)
    };

    if code < 0 {
        return ThreadName::new();
    }

    let mut result = [0u8; 16];
    unsafe {
        WideCharToMultiByte(CP_UTF8, 0, desc_ptr as _, -1, result.as_mut_ptr() as _, result.len() as _, ptr::null(), ptr::null_mut());
        LocalFree(desc_ptr as _);
    };

    ThreadName::name(result)
}
