
extern crate libc;
extern crate libvirt_sys;

use std::error::Error as StdError;
use std::fmt::{Display, Result as FmtResult, Formatter};

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
    pub fn new() -> Error {
        unsafe {
            let ptr: libvirt_sys::virErrorPtr = libvirt_sys::virGetLastError();
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
