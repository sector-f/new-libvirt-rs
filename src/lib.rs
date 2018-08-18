macro_rules! c_chars_to_string {
    ($x:expr) => {{
        let ret = ::std::ffi::CStr::from_ptr($x).to_string_lossy().into_owned();
        libc::free($x as *mut libc::c_void);
        ret
    }};

    ($x:expr, nofree) => {{
        ::std::ffi::CStr::from_ptr($x).to_string_lossy().into_owned()
    }};

}

macro_rules! string_to_c_chars {
    ($x:expr) => (::std::ffi::CString::new($x).unwrap().as_ptr())
}

#[allow(unused_macros)]
macro_rules! string_to_mut_c_chars {
    ($x:expr) => (::std::ffi::CString::new($x).unwrap().into_raw())
}

#[allow(unused_macros)]
macro_rules! string_to_mut_c_chars {
    ($x:expr) => (::std::ffi::CString::new($x).unwrap().into_raw())
}

macro_rules! impl_from {
    // Largely inspired by impl_from! in rust core/num/mod.rs
    ($Small: ty, $Large: ty) => {
        impl From<$Small> for $Large {
            #[inline]
            fn from(small: $Small) -> $Large {
                let r: $Large;
                unsafe {
                    r = ::std::mem::transmute(small)
                }
                r
            }
        }
    }
}

pub mod connect;
pub mod domain;
pub mod error;
