extern crate libc;
extern crate libvirt_sys as sys;
use error::Error;

use connect::Connect;
// use std::ffi::CStr;
// use std::{ptr, slice, mem};
// use std::os::raw::c_int;

pub mod flags;
use interface::flags::*;

pub struct Interface {
    ptr: Option<sys::virInterfacePtr>,
}

impl Drop for Interface {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for Interface, code {}, message: {}",
                       e.code,
                       e.message)
            }
        }
    }
}

impl Interface {
    pub fn new(ptr: sys::virInterfacePtr) -> Interface {
        return Interface { ptr: Some(ptr) };
    }

    pub fn as_ptr(&self) -> sys::virInterfacePtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = sys::virInterfaceGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Connect::new(ptr));
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Interface, Error> {
        unsafe {
            let ptr = sys::virInterfaceLookupByName(conn.as_ptr(), string_to_c_chars!(id));
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Interface::new(ptr));
        }
    }

    pub fn define_xml(conn: &Connect, xml: &str) -> Result<Interface, Error> {
        unsafe {
            let ptr = sys::virInterfaceDefineXML(conn.as_ptr(), string_to_c_chars!(xml), 0);
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Interface::new(ptr));
        }
    }

    pub fn lookup_by_mac_string(conn: &Connect, id: &str) -> Result<Interface, Error> {
        unsafe {
            let ptr = sys::virInterfaceLookupByMACString(conn.as_ptr(), string_to_c_chars!(id));
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Interface::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virInterfaceGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(n, nofree));
        }
    }

    pub fn get_mac_string(&self) -> Result<String, Error> {
        unsafe {
            let mac = sys::virInterfaceGetMACString(self.as_ptr());
            if mac.is_null() {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(mac, nofree));
        }
    }

    pub fn get_xml_desc(&self, flags: InterfaceXmlFlags) -> Result<String, Error> {
        unsafe {
            let xml = sys::virInterfaceGetXMLDesc(self.as_ptr(), flags.bits());
            if xml.is_null() {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(xml));
        }
    }

    pub fn create(&self) -> Result<(), Error> {
        unsafe {
            let ret = sys::virInterfaceCreate(self.as_ptr(), 0);
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if sys::virInterfaceDestroy(self.as_ptr(), 0) == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if sys::virInterfaceUndefine(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }


    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::virInterfaceFree(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            self.ptr = None;
            return Ok(());
        }
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virInterfaceIsActive(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(ret == 1);
        }
    }
}
