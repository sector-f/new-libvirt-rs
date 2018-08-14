extern crate libc;
extern crate libvirt_sys;

use domain::{Domain, ListAllDomainsFlags};
use error::Error;
use std::{ptr, mem};

#[derive(Debug)]
pub struct Connect {
    ptr: Option<libvirt_sys::virConnectPtr>,
}

impl Connect {
    pub fn as_ptr(&self) -> libvirt_sys::virConnectPtr {
        self.ptr.unwrap()
    }

    pub fn new(ptr: libvirt_sys::virConnectPtr) -> Connect {
        return Connect { ptr: Some(ptr) };
    }

    pub fn open(uri: &str) -> Result<Connect, Error> {
        unsafe {
            let c = libvirt_sys::virConnectOpen(string_to_c_chars!(uri));
            if c.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect::new(c));
        }
    }

    pub fn open_read_only(uri: &str) -> Result<Connect, Error> {
        unsafe {
            let c = libvirt_sys::virConnectOpenReadOnly(string_to_c_chars!(uri));
            if c.is_null() {
                return Err(Error::new());
            }
            return Ok(Connect::new(c));
        }
    }

    pub fn list_all_domains(&self, flags: &[ListAllDomainsFlags])-> Result<Vec<Domain>, Error> {
        let flags_value = flags.iter().fold(0, |acc, flag| acc + *flag as u32);

        unsafe {
            let mut domains: *mut libvirt_sys::virDomainPtr = ptr::null_mut();
            let size = libvirt_sys::virConnectListAllDomains(self.as_ptr(), &mut domains, flags_value as libc::c_uint);
            if size == -1 {
                return Err(Error::new());
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
}
