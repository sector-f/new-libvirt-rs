extern crate libc;
extern crate libvirt_sys as sys;

use connect::Connect;
use storage_vol::StorageVol;
use error::Error;

pub mod flags;
use storage_pool::flags::*;

#[derive(Debug)]
pub struct StoragePoolInfo {
    pub state: StoragePoolState,
    /// Logical size bytes.
    pub capacity: u64,
    /// Current allocation bytes.
    pub allocation: u64,
    /// Remaining free space bytes.
    pub available: u64,
}

impl StoragePoolInfo {
    pub fn from_ptr(ptr: sys::virStoragePoolInfoPtr) -> StoragePoolInfo {
        unsafe {
            StoragePoolInfo {
                state: StoragePoolState::from_int((*ptr).state).unwrap(),
                capacity: (*ptr).capacity as u64,
                allocation: (*ptr).allocation as u64,
                available: (*ptr).available as u64,
            }
        }
    }
}

pub struct StoragePool {
    ptr: Option<sys::virStoragePoolPtr>,
}

impl Drop for StoragePool {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for StoragePool, code {}, message: {}", e.code, e.message)
            }
        }
    }
}

impl StoragePool {
    pub fn new(ptr: sys::virStoragePoolPtr) -> StoragePool {
        return StoragePool { ptr: Some(ptr) };
    }

    pub fn as_ptr(&self) -> sys::virStoragePoolPtr {
        self.ptr.unwrap()
    }

    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = sys::virStoragePoolGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Connect::new(ptr));
        }
    }
    pub fn define_xml(conn: &Connect, xml: &str) -> Result<StoragePool, Error> {
        unsafe {
            let ptr = sys::virStoragePoolDefineXML(conn.as_ptr(), string_to_c_chars!(xml), 0);
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(StoragePool::new(ptr));
        }
    }

    pub fn create_xml(conn: &Connect, xml: &str, flags: StoragePoolCreateFlags) -> Result<StoragePool, Error> {
        unsafe {
            let ptr = sys::virStoragePoolCreateXML(conn.as_ptr(), string_to_c_chars!(xml), flags.bits());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(StoragePool::new(ptr));
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<StoragePool, Error> {
        unsafe {
            let ptr = sys::virStoragePoolLookupByName(conn.as_ptr(), string_to_c_chars!(id));
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(StoragePool::new(ptr));
        }
    }

    pub fn lookup_by_volume(vol: &StorageVol) -> Result<StoragePool, Error> {
        unsafe {
            let ptr = sys::virStoragePoolLookupByVolume(vol.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(StoragePool::new(ptr));
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<StoragePool, Error> {
        unsafe {
            let ptr = sys::virStoragePoolLookupByUUIDString(conn.as_ptr(), string_to_c_chars!(uuid));
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(StoragePool::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virStoragePoolGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(n, nofree));
        }
    }

    pub fn num_of_volumes(&self) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virStoragePoolNumOfVolumes(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(ret as u32);
        }
    }

    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if sys::virStoragePoolGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) == -1 {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(uuid.as_ptr(), nofree));
        }
    }

    pub fn get_xml_desc(&self, flags: StoragePoolXmlFlags) -> Result<String, Error> {
        unsafe {
            let xml = sys::virStoragePoolGetXMLDesc(self.as_ptr(), flags.bits());
            if xml.is_null() {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(xml));
        }
    }

    pub fn create(&self, flags: StoragePoolCreateFlags) -> Result<(), Error> {
        unsafe {
            let ret = sys::virStoragePoolCreate(self.as_ptr(), flags.bits());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn build(&self, flags: StoragePoolBuildFlags) -> Result<(), Error> {
        unsafe {
            let ret = sys::virStoragePoolBuild(self.as_ptr(), flags.bits());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if sys::virStoragePoolDestroy(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn delete(&self, flags: StoragePoolDeleteFlags) -> Result<(), Error> {
        unsafe {
            if sys::virStoragePoolDelete(self.as_ptr(), flags.bits()) == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if sys::virStoragePoolUndefine(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::virStoragePoolFree(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            self.ptr = None;
            return Ok(());
        }
    }

    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virStoragePoolIsActive(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(ret == 1);
        }
    }

    pub fn is_persistent(&self) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virStoragePoolIsPersistent(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(ret == 1);
        }
    }

    pub fn refresh(&self) -> Result<(), Error> {
        unsafe {
            let ret = sys::virStoragePoolRefresh(self.as_ptr(), 0);
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }


    pub fn get_autostart(&self) -> Result<bool, Error> {
        unsafe {
            let mut auto = 0;
            let ret = sys::virStoragePoolGetAutostart(self.as_ptr(), &mut auto);
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(auto == 1);
        }
    }

    pub fn set_autostart(&self, autostart: bool) -> Result<(), Error> {
        unsafe {
            let ret = sys::virStoragePoolSetAutostart(self.as_ptr(), autostart as libc::c_int);
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn get_info(&self) -> Result<StoragePoolInfo, Error> {
        unsafe {
            let pinfo = &mut sys::virStoragePoolInfo::default();
            let res = sys::virStoragePoolGetInfo(self.as_ptr(), pinfo);
            if res == -1 {
                return Err(Error::last_error());
            }
            return Ok(StoragePoolInfo::from_ptr(pinfo));
        }
    }
}
