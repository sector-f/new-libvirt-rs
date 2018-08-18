extern crate libc;
extern crate libvirt_sys as sys;

use std::error::Error as StdError;
use std::fmt::{Display, Result as FmtResult, Formatter};
use std::os::raw::c_void;
use std::ptr::NonNull;

#[derive(Debug, PartialEq)]
#[repr(C)]
pub enum ErrorLevel {
    NONE = 0,
    /// A simple warning.
    WARNING = 1,
    /// An error.
    ERROR = 2,
}
impl_from! { u32, ErrorLevel }

/// Error handling
///
/// See: http://libvirt.org/html/libvirt-virterror.html
#[derive(Debug, PartialEq)]
pub struct Error {
    pub code: i32,
    pub domain: i32,
    pub message: String,
    pub level: ErrorLevel,
}

impl Error {
    /// Returns the most-recent libvirt error
    pub fn last_error() -> Error {
        unsafe {
            let ptr: sys::virErrorPtr = sys::virGetLastError();
            Error::from_ptr(ptr)
        }
    }

    pub fn from_ptr(ptr: sys::virErrorPtr) -> Error {
        unsafe {
            Error {
                code: (*ptr).code,
                domain: (*ptr).domain,
                message: c_chars_to_string!((*ptr).message, nofree),
                level: ErrorLevel::from((*ptr).level),
            }
        }
    }
}

impl StdError for Error {
    fn description(&self) -> &str {
        self.message.as_str()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f,
               "{:?}: code: {} domain: {} - {}",
               self.level,
               self.code,
               self.domain,
               self.message)
    }
}

static mut HANDLER: Option<NonNull<Fn(Error)>> = None;

pub fn set_error_func<F: Fn(Error) + 'static>(f: F) {
    unsafe extern "C" fn callback<F: Fn(Error)>(user_data: *mut c_void, error: sys::virErrorPtr) {
        let f = user_data as *const F;
        let wrapped_error = Error::from_ptr(error);
        (*f)(wrapped_error)
    }
    let data = Box::into_raw(Box::new(f));
    unsafe {
        HANDLER.take().map(|p| Box::from_raw(p.as_ptr())); //drop the old one
        HANDLER = Some(NonNull::new_unchecked(data));
        sys::virSetErrorFunc(data as _, Some(callback::<F>))
    }
}
