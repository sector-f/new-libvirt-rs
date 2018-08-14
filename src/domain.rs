extern crate libvirt_sys;
use error::Error;

use std::ffi::CStr;
use std::{ptr, slice};

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

#[derive(Clone, Debug)]
pub struct InterfaceStats {
    pub rx_bytes: i64,
    pub rx_packets: i64,
    pub rx_errs: i64,
    pub rx_drop: i64,
    pub tx_bytes: i64,
    pub tx_packets: i64,
    pub tx_errs: i64,
    pub tx_drop: i64,
}

impl InterfaceStats {
    pub fn from_ptr(ptr: libvirt_sys::virDomainInterfaceStatsPtr) -> InterfaceStats {
        unsafe {
            InterfaceStats {
                rx_bytes: (*ptr).rx_bytes as i64,
                rx_packets: (*ptr).rx_packets as i64,
                rx_errs: (*ptr).rx_errs as i64,
                rx_drop: (*ptr).rx_drop as i64,
                tx_bytes: (*ptr).tx_bytes as i64,
                tx_packets: (*ptr).tx_packets as i64,
                tx_errs: (*ptr).tx_errs as i64,
                tx_drop: (*ptr).tx_drop as i64,
            }
        }
    }
}

pub enum InterfaceAddressSource {
    Lease = 0,
    Agent = 1,
    Arp = 2,
    Last = 3,
}

#[derive(Debug)]
pub enum IpAddrType {
    V4 = 0,
    V6 = 1,
    Last = 2,
}

#[derive(Debug)]
pub struct DomainIpAddress {
    // pub type_: IpAddrType,
    pub addr: String,
    pub prefix: u32,
}

#[derive(Debug)]
pub struct DomainInterface {
    pub name: String,
    pub hwaddr: String,
    pub addrs: Vec<DomainIpAddress>,
}

pub struct Domain {
    ptr: Option<libvirt_sys::virDomainPtr>,
}

impl Domain {
    pub fn new(ptr: libvirt_sys::virDomainPtr) -> Self {
        return Domain { ptr: Some(ptr) };
    }

    pub fn as_ptr(&self) -> libvirt_sys::virDomainPtr {
        self.ptr.unwrap()
    }

    pub fn interface_addresses(&self, source: InterfaceAddressSource) -> Result<Vec<DomainInterface>, Error> {
        let mut interfaces: Vec<DomainInterface> = Vec::new();
        let mut iface_ptr: *mut libvirt_sys::virDomainInterfacePtr = ptr::null_mut();

        unsafe {
            let ifaces_count = libvirt_sys::virDomainInterfaceAddresses(self.as_ptr(), &mut iface_ptr, source as u32, 0);
            if ifaces_count == -1 {
                return Err(Error::new());
            }

            let ifaces = slice::from_raw_parts::<*mut libvirt_sys::virDomainInterfacePtr>(&mut iface_ptr, ifaces_count as usize);

            for iface in ifaces {
                let name = String::from_utf8_lossy(CStr::from_ptr((***iface).name).to_bytes()).into_owned();
                let hwaddr = String::from_utf8_lossy(CStr::from_ptr((***iface).hwaddr).to_bytes()).into_owned();

                let raw_addrs = slice::from_raw_parts::<libvirt_sys::virDomainIPAddressPtr>(&mut (***iface).addrs, (***iface).naddrs as usize);
                let addresses = raw_addrs.into_iter()
                    .map(|a| DomainIpAddress {
                        // type_: (**a).type_,
                        addr: String::from_utf8_lossy(CStr::from_ptr((**a).addr).to_bytes()).into_owned(),
                        prefix: (**a).prefix
                    }).collect::<Vec<DomainIpAddress>>();

                interfaces.push(
                    DomainInterface {
                        name: name,
                        hwaddr: hwaddr,
                        addrs: addresses,
                    }
                );
            }
        }

        Ok(interfaces)
    }
}
