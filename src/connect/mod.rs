extern crate libc;
extern crate libvirt_sys as sys;

use domain::Domain;
use error::Error;
use std::{ptr, mem};

pub mod flags;
use connect::flags::*;

#[derive(Debug)]
pub struct Connect {
    ptr: Option<sys::virConnectPtr>,
}

impl Connect {
    pub fn as_ptr(&self) -> sys::virConnectPtr {
        self.ptr.unwrap()
    }

    pub fn new(ptr: sys::virConnectPtr) -> Connect {
        return Connect { ptr: Some(ptr) };
    }

    pub fn open(uri: &str) -> Result<Connect, Error> {
        unsafe {
            let c = sys::virConnectOpen(string_to_c_chars!(uri));
            if c.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Connect::new(c));
        }
    }

    pub fn open_read_only(uri: &str) -> Result<Connect, Error> {
        unsafe {
            let c = sys::virConnectOpenReadOnly(string_to_c_chars!(uri));
            if c.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Connect::new(c));
        }
    }

    pub fn list_all_domains(&self, flags: Option<ListAllDomainsFlags>)-> Result<Vec<Domain>, Error> {
        let flags_value = flags.and_then(|f| Some(f.bits())).unwrap_or(0);

        unsafe {
            let mut domains: *mut sys::virDomainPtr = ptr::null_mut();
            let size = sys::virConnectListAllDomains(self.as_ptr(), &mut domains, flags_value as libc::c_uint);
            if size == -1 {
                return Err(Error::last_error());
            }

            mem::forget(domains);

            let mut array: Vec<Domain> = Vec::new();
            for x in 0..size as isize {
                array.push(Domain::new(*domains.offset(x)));
            }
            libc::free(domains as *mut libc::c_void);

            return Ok(array);
        }
    }

    pub fn list_active_domains(&self) -> Result<Vec<u32>, Error> {
        unsafe {
            let mut ids: [libc::c_int; 512] = [0; 512];
            let size = sys::virConnectListDomains(self.as_ptr(), ids.as_mut_ptr(), 512);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<u32> = Vec::new();
            for x in 0..size as usize {
                array.push(ids[x] as u32);
            }
            return Ok(array);
        }
    }

    pub fn list_defined_domains(&self) -> Result<Vec<String>, Error> {
        unsafe {
            let mut names: [*mut libc::c_char; 1024] = [ptr::null_mut(); 1024];
            let size = sys::virConnectListDefinedDomains(self.as_ptr(), names.as_mut_ptr(), 1024);
            if size == -1 {
                return Err(Error::last_error());
            }

            let mut array: Vec<String> = Vec::new();
            for x in 0..size as usize {
                array.push(c_chars_to_string!(names[x]));
            }
            return Ok(array);
        }
    }
}
