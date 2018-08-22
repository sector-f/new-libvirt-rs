extern crate libc;
extern crate libvirt_sys as sys;
use error::Error;

use connect::Connect;
use std::ffi::CStr;
use std::{ptr, slice, mem};
use std::os::raw::c_int;

pub mod flags;
use storage_vol::flags::*;

