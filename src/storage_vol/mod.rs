extern crate libc;
extern crate libvirt_sys as sys;
use error::Error;

use connect::Connect;
use stream::Stream;

pub mod flags;
use storage_vol::flags::*;

#[derive(Clone, Debug)]
pub struct StorageVolInfo {
    /// See: `virStorageVolType` flags
    pub type_: StorageVolType,
    /// Logical size bytes.
    pub capacity: u64,
    /// Current allocation bytes
    pub allocation: u64,
}

impl Default for StorageVolInfo {
    fn default() -> Self {
        StorageVolInfo {
            type_: StorageVolType::File,
            capacity: 0,
            allocation: 0,
        }
    }
}

impl StorageVolInfo {
    pub fn from_ptr(ptr: sys::virStorageVolInfoPtr) -> StorageVolInfo {
        unsafe {
            StorageVolInfo {
                type_: StorageVolType::from_int((*ptr).type_).unwrap(),
                capacity: (*ptr).capacity as u64,
                allocation: (*ptr).allocation as u64,
            }
        }
    }
}

/// Provides APIs for the management of storage volumes.
///
/// See http://libvirt.org/html/libvirt-libvirt-storage.html
#[derive(Debug)]
pub struct StorageVol {
    ptr: Option<sys::virStorageVolPtr>,
}

impl Drop for StorageVol {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for StorageVol, code {}, message: {}",
                       e.code,
                       e.message)
            }
        }
    }
}

impl StorageVol {
    pub fn new(ptr: sys::virStorageVolPtr) -> StorageVol {
        return StorageVol { ptr: Some(ptr) };
    }

    pub fn as_ptr(&self) -> sys::virStorageVolPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = sys::virStorageVolGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Connect::new(ptr));
        }
    }

    // TODO: uncomment these once StoragePool is implemented

    // pub fn create_xml(pool: &StoragePool, xml: &str, flags: StorageVolCreateFlags) -> Result<StorageVol, Error> {
    //     unsafe {
    //         let ptr = sys::virStorageVolCreateXML(pool.as_ptr(), string_to_c_chars!(xml), flags.bits());
    //         if ptr.is_null() {
    //             return Err(Error::last_error());
    //         }
    //         return Ok(StorageVol::new(ptr));
    //     }
    // }

    // pub fn create_xml_from(pool: &StoragePool, xml: &str, vol: &StorageVol, flags: StorageVolCreateFlags) -> Result<StorageVol, Error> {
    //     unsafe {
    //         let ptr = sys::virStorageVolCreateXMLFrom(pool.as_ptr(), string_to_c_chars!(xml), vol.as_ptr(), flags.bits());
    //         if ptr.is_null() {
    //             return Err(Error::last_error());
    //         }
    //         return Ok(StorageVol::new(ptr));
    //     }
    // }

    // pub fn lookup_by_name(pool: &StoragePool, name: &str) -> Result<StorageVol, Error> {
    //     unsafe {
    //         let ptr = sys::virStorageVolLookupByName(pool.as_ptr(), string_to_c_chars!(name));
    //         if ptr.is_null() {
    //             return Err(Error::last_error());
    //         }
    //         return Ok(StorageVol::new(ptr));
    //     }
    // }

    pub fn lookup_by_key(conn: &Connect, key: &str) -> Result<StorageVol, Error> {
        unsafe {
            let ptr = sys::virStorageVolLookupByKey(conn.as_ptr(), string_to_c_chars!(key));
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(StorageVol::new(ptr));
        }
    }

    pub fn lookup_by_path(conn: &Connect, path: &str) -> Result<StorageVol, Error> {
        unsafe {
            let ptr = sys::virStorageVolLookupByPath(conn.as_ptr(), string_to_c_chars!(path));
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(StorageVol::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virStorageVolGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(n, nofree));
        }
    }

    pub fn get_key(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virStorageVolGetKey(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(n, nofree));
        }
    }

    pub fn get_path(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virStorageVolGetPath(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    pub fn get_xml_desc(&self) -> Result<String, Error> {
        unsafe {
            let xml = sys::virStorageVolGetXMLDesc(self.as_ptr(), 0);
            if xml.is_null() {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(xml));
        }
    }

    pub fn delete(&self, flags: StorageVolDeleteFlags) -> Result<(), Error> {
        unsafe {
            if sys::virStorageVolDelete(self.as_ptr(), flags.bits()) == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn wipe(&self) -> Result<(), Error> {
        unsafe {
            if sys::virStorageVolWipe(self.as_ptr(), 0) == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn wipe_pattern(&self, algo: StorageVolWipeAlgorithm) -> Result<(), Error> {
        unsafe {
            if sys::virStorageVolWipePattern(self.as_ptr(), algo as libc::c_uint, 0) == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::virStorageVolFree(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            self.ptr = None;
            return Ok(());
        }
    }

    pub fn resize(&self, capacity: u64, flags: StorageVolResizeFlags) -> Result<(), Error> {
        unsafe {
            let ret = sys::virStorageVolResize(self.as_ptr(), capacity as libc::c_ulonglong, flags.bits());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn get_info(&self) -> Result<StorageVolInfo, Error> {
        unsafe {
            let pinfo = &mut sys::virStorageVolInfo::default();
            let res = sys::virStorageVolGetInfo(self.as_ptr(), pinfo);
            if res == -1 {
                return Err(Error::last_error());
            }
            return Ok(StorageVolInfo::from_ptr(pinfo));
        }
    }

    pub fn get_info_flags(&self, flags: StorageVolInfoFlags) -> Result<StorageVolInfo, Error> {
        unsafe {
            let pinfo = &mut sys::virStorageVolInfo::default();
            let res = sys::virStorageVolGetInfoFlags(self.as_ptr(), pinfo, flags.bits());
            if res == -1 {
                return Err(Error::last_error());
            }
            return Ok(StorageVolInfo::from_ptr(pinfo));
        }
    }

    pub fn download(&self, stream: &Stream, offset: u64, length: u64, flags: StorageVolDownloadFlags) -> Result<(), Error> {
        unsafe {
            let ret = sys::virStorageVolDownload(self.as_ptr(), stream.as_ptr(), offset as libc::c_ulonglong, length as libc::c_ulonglong, flags.bits());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn upload(&self, stream: &Stream, offset: u64, length: u64, flags: StorageVolUploadFlags) -> Result<(), Error> {
        unsafe {
            let ret = sys::virStorageVolUpload(self.as_ptr(), stream.as_ptr(), offset as libc::c_ulonglong, length as libc::c_ulonglong, flags.bits());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }
}
