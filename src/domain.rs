extern crate libc;
extern crate libvirt_sys as sys;
use error::Error;

use connect::Connect;
use std::ffi::CStr;
use std::{ptr, slice, mem};

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
    pub fn from_ptr(ptr: sys::virDomainInterfaceStatsPtr) -> InterfaceStats {
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

#[derive(Clone, Debug)]
pub struct DomainInfo {
    /// The running state, one of virDomainState.
    pub state: DomainState,
    /// The maximum memory in KBytes allowed.
    pub max_mem: u64,
    /// The memory in KBytes used by the domain.
    pub memory: u64,
    /// The number of virtual CPUs for the domain.
    pub nr_virt_cpu: u32,
    /// The CPU time used in nanoseconds.
    pub cpu_time: u64,
}

impl DomainInfo {
    pub fn from_ptr(ptr: sys::virDomainInfoPtr) -> DomainInfo {
        unsafe {
            DomainInfo {
                state: DomainState::new((*ptr).state).unwrap(),
                max_mem: (*ptr).maxMem as u64,
                memory: (*ptr).memory as u64,
                nr_virt_cpu: (*ptr).nrVirtCpu as u32,
                cpu_time: (*ptr).cpuTime as u64,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum DomainState {
    NoState = 0,
    Running = 1,
    Blocked = 2,
    Paused = 3,
    Shutdown = 4,
    Shutoff = 5,
    Crashed = 6,
    PmSuspended = 7,
}

impl DomainState {
    pub fn new(n: u8) -> Option<Self> {
        match n {
            0 => Some(DomainState::NoState),
            1 => Some(DomainState::Running),
            2 => Some(DomainState::Blocked),
            3 => Some(DomainState::Paused),
            4 => Some(DomainState::Shutdown),
            5 => Some(DomainState::Shutoff),
            6 => Some(DomainState::Crashed),
            7 => Some(DomainState::PmSuspended),
            _ => None,
        }
    }
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
    ptr: Option<sys::virDomainPtr>,
}

impl Domain {
    pub fn new(ptr: sys::virDomainPtr) -> Self {
        return Domain { ptr: Some(ptr) };
    }

    pub fn as_ptr(&self) -> sys::virDomainPtr {
        self.ptr.unwrap()
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<Domain, Error> {
        unsafe {
            let ptr = sys::virDomainLookupByID(conn.as_ptr(), id as libc::c_int);
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Domain, Error> {
        unsafe {
            let ptr = sys::virDomainLookupByName(conn.as_ptr(), string_to_c_chars!(id));
            if ptr.is_null() {
                return Err(Error::new());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virDomainGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::new());
            }
            return Ok(c_chars_to_string!(n, nofree));
        }
    }

    pub fn get_info(&self) -> Result<DomainInfo, Error> {
        unsafe {
            let pinfo = &mut sys::virDomainInfo::default();
            let res = sys::virDomainGetInfo(self.as_ptr(), pinfo);
            if res == -1 {
                return Err(Error::new());
            }
            return Ok(DomainInfo::from_ptr(pinfo));
        }
    }

    pub fn get_state(&self) -> Result<(DomainState, i32), Error> {
        unsafe {
            let mut state: libc::c_int = -1;
            let mut reason: libc::c_int = -1;
            let ret = sys::virDomainGetState(self.as_ptr(), &mut state, &mut reason, 0);
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok((DomainState::new(state as u8).unwrap(), reason as i32));
        }
    }

    pub fn interface_stats(&self, path: &str) -> Result<InterfaceStats, Error> {
        unsafe {
            let pinfo = &mut sys::_virDomainInterfaceStats::default();
            let ret = sys::virDomainInterfaceStats(self.as_ptr(),
                                              string_to_c_chars!(path),
                                              pinfo,
                                              mem::size_of::<sys::_virDomainInterfaceStats>());
            if ret == -1 {
                return Err(Error::new());
            }
            return Ok(InterfaceStats::from_ptr(pinfo));
        }
    }

    pub fn interface_addresses(&self, source: InterfaceAddressSource) -> Result<Vec<DomainInterface>, Error> {
        let mut interfaces: Vec<DomainInterface> = Vec::new();
        let mut iface_ptr: *mut sys::virDomainInterfacePtr = ptr::null_mut();

        unsafe {
            let ifaces_count = sys::virDomainInterfaceAddresses(self.as_ptr(), &mut iface_ptr, source as u32, 0);
            if ifaces_count == -1 {
                return Err(Error::new());
            }

            let ifaces = slice::from_raw_parts::<*mut sys::virDomainInterfacePtr>(&mut iface_ptr, ifaces_count as usize);

            for iface in ifaces {
                let name = String::from_utf8_lossy(CStr::from_ptr((***iface).name).to_bytes()).into_owned();
                let hwaddr = String::from_utf8_lossy(CStr::from_ptr((***iface).hwaddr).to_bytes()).into_owned();

                let raw_addrs = slice::from_raw_parts::<sys::virDomainIPAddressPtr>(&mut (***iface).addrs, (***iface).naddrs as usize);
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
