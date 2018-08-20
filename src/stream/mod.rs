extern crate libc;
extern crate libvirt_sys as sys;
use error::Error;

pub mod flags;
// use stream::flags::*;

#[derive(Debug)]
pub struct Stream {
    ptr: Option<sys::virStreamPtr>,
}

impl Drop for Stream {
    fn drop(&mut self) {
        if self.ptr.is_some() {
            if let Err(e) = self.free() {
                panic!("Unable to drop memory for Stream, code {}, message: {}", e.code, e.message)
            }
        }
    }
}

impl Stream {
    pub fn new(ptr: sys::virStreamPtr) -> Stream {
        Stream { ptr: Some(ptr) }
    }

    pub fn as_ptr(&self) -> sys::virStreamPtr {
        self.ptr.unwrap()
    }

    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::virStreamFree(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            self.ptr = None;
            return Ok(());
        }
    }

    pub fn finish(self) -> Result<(), Error> {
        unsafe {
            if sys::virStreamFinish(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn abort(self) -> Result<(), Error> {
        unsafe {
            if sys::virStreamAbort(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn send(&self, data: &str) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virStreamSend(self.as_ptr(), string_to_c_chars!(data), data.len());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(ret as u32);
        }
    }

    pub fn recv(&self, size: usize) -> Result<String, Error> {
        unsafe {
            let mut data: [libc::c_char; 2048] = ['\0' as i8; 2048];
            let ret = sys::virStreamRecv(self.as_ptr(), data.as_mut_ptr(), size);
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(data.as_ptr()));
        }
    }
}
