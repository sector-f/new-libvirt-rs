extern crate libc;
extern crate libvirt_sys as sys;
use error::Error;

use connect::Connect;

pub mod flags;
use secret::flags::*;

/// Provides APIs for the management of secrets.
///
/// See http://libvirt.org/html/libvirt-libvirt-secret.html
#[derive(Debug)]
pub struct Secret {
    ptr: Option<sys::virSecretPtr>,
}

impl Secret {
    pub fn new(ptr: sys::virSecretPtr) -> Secret {
        return Secret { ptr: Some(ptr) };
    }

    pub fn as_ptr(&self) -> sys::virSecretPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = sys::virSecretGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Connect::new(ptr));
        }
    }

    pub fn define_xml(conn: &Connect, xml: &str) -> Result<Secret, Error> {
        unsafe {
            let ptr = sys::virSecretDefineXML(conn.as_ptr(),
                                         string_to_c_chars!(xml),
                                         0);
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Secret::new(ptr));
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Secret, Error> {
        unsafe {
            let ptr = sys::virSecretLookupByUUIDString(conn.as_ptr(), string_to_c_chars!(uuid));
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Secret::new(ptr));
        }
    }

    pub fn lookup_by_usage(conn: &Connect, usagetype: SecretUsageType, usageid: &str) -> Result<Secret, Error> {
        unsafe {
            let ptr = sys::virSecretLookupByUsage(conn.as_ptr(),
                                             usagetype.bits() as i32,
                                             string_to_c_chars!(usageid));
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Secret::new(ptr));
        }
    }

    pub fn get_usage_id(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virSecretGetUsageID(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    pub fn get_usage_type(&self) -> Result<SecretUsageType, Error> {
        unsafe {
            let t = sys::virSecretGetUsageType(self.as_ptr());
            if t == -1 {
                return Err(Error::last_error());
            }
            return Ok(SecretUsageType::from_bits_truncate(t as u32));
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if sys::virSecretGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) == -1 {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(uuid.as_ptr(), nofree));
        }
    }

    pub fn get_xml_desc(&self, flags: SecretXMLFlags) -> Result<String, Error> {
        unsafe {
            let xml = sys::virSecretGetXMLDesc(self.as_ptr(), flags.bits());
            if xml.is_null() {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(xml));
        }
    }

    pub fn set_value(&self, value: &[u8]) -> Result<(), Error> {
        unsafe {
            if sys::virSecretSetValue(self.as_ptr(),
                                 value.as_ptr(),
                                 value.len(),
                                 0) == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn get_value(&self, size: isize) -> Result<Vec<u8>, Error> {
        unsafe {
            let size_ptr: *mut usize = &mut (size as usize);
            let n = sys::virSecretGetValue(self.as_ptr(), size_ptr, 0);
            if n.is_null() {
                return Err(Error::last_error());
            }

            let mut array: Vec<u8> = Vec::new();
            for x in 0..size {
                array.push(*n.offset(x))
            }
            return Ok(array);
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if sys::virSecretUndefine(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::virSecretFree(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            self.ptr = None;
            return Ok(());
        }
    }
}
