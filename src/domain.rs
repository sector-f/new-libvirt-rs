extern crate libvirt_sys;

#[derive(Copy, Clone)]
pub enum ListAllDomainsFlags {
    All = 0,
    Active = 1,
    Inactive = 2,
    Persistent = 4,
    Transient = 8,
    Running = 16,
    Paused = 32,
    Shutoff = 64,
    Other = 128,
    ManagedSave = 256,
    NoManagedSave = 512,
    Autostart = 1024,
    NoAutostart = 2048,
    HasSnapshot = 4096,
    NoSnapshot = 8192,
}

pub struct Domain {
    ptr: Option<libvirt_sys::virDomainPtr>,
}

impl Domain {
    pub fn new(ptr: libvirt_sys::virDomainPtr) -> Self {
        return Domain { ptr: Some(ptr) };
    }
}
