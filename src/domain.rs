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

#[derive(Copy, Clone)]
pub enum DomainCreateFlags {
    None = 0,
    StartPaused = 1,
    StartAutodestroy = 2,
    StartBypassCache = 4,
    StartForceBoot = 8,
    StartValidate = 16,
}

#[derive(Copy, Clone)]
pub enum XmlFlags {
    XmlSecure = 1,
    XmlInactive = 2,
    XmlUpdateCpu = 4,
    XmlMigratable = 8,
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


    pub fn get_connect(&self) -> Result<Connect, Error> {
        unsafe {
            let ptr = sys::virDomainGetConnect(self.as_ptr());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Connect::new(ptr));
        }
    }

    pub fn lookup_by_id(conn: &Connect, id: u32) -> Result<Domain, Error> {
        unsafe {
            let ptr = sys::virDomainLookupByID(conn.as_ptr(), id as libc::c_int);
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn lookup_by_name(conn: &Connect, id: &str) -> Result<Domain, Error> {
        unsafe {
            let ptr = sys::virDomainLookupByName(conn.as_ptr(), string_to_c_chars!(id));
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn lookup_by_uuid_string(conn: &Connect, uuid: &str) -> Result<Domain, Error> {
        unsafe {
            let ptr = sys::virDomainLookupByUUIDString(conn.as_ptr(), string_to_c_chars!(uuid));
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Domain::new(ptr));
        }
    }

    pub fn get_name(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virDomainGetName(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(n, nofree));
        }
    }

    pub fn get_info(&self) -> Result<DomainInfo, Error> {
        unsafe {
            let pinfo = &mut sys::virDomainInfo::default();
            let res = sys::virDomainGetInfo(self.as_ptr(), pinfo);
            if res == -1 {
                return Err(Error::last_error());
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
                return Err(Error::last_error());
            }
            return Ok((DomainState::new(state as u8).unwrap(), reason as i32));
        }
    }

    /// Get the type of domain operating system.
    pub fn get_os_type(&self) -> Result<String, Error> {
        unsafe {
            let n = sys::virDomainGetOSType(self.as_ptr());
            if n.is_null() {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    /// Get the hostname for that domain.
    pub fn get_hostname(&self, flags: u32) -> Result<String, Error> {
        unsafe {
            let n = sys::virDomainGetHostname(self.as_ptr(), flags as libc::c_uint);
            if n.is_null() {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(n));
        }
    }

    /// Get the UUID for a domain as string.
    ///
    /// For more information about UUID see RFC4122.
    pub fn get_uuid_string(&self) -> Result<String, Error> {
        unsafe {
            let mut uuid: [libc::c_char; 37] = [0; 37];
            if sys::virDomainGetUUIDString(self.as_ptr(), uuid.as_mut_ptr()) == -1 {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(uuid.as_ptr(), nofree));
        }
    }

    /// Get the hypervisor ID number for the domain
    pub fn get_id(&self) -> Option<u32> {
        unsafe {
            let ret = sys::virDomainGetID(self.as_ptr());
            if ret as i32 == -1 {
                return None;
            }
            Some(ret)
        }
    }

    /// Provide an XML description of the domain. The description may
    /// be reused later to relaunch the domain with `create_xml()`.
    pub fn get_xml_desc(&self, flags: &[XmlFlags]) -> Result<String, Error> {
        let bitfield: u32 = flags.iter().fold(0, |acc, x| acc | *x as u32);

        unsafe {
            let xml = sys::virDomainGetXMLDesc(self.as_ptr(), bitfield);
            if xml.is_null() {
                return Err(Error::last_error());
            }
            return Ok(c_chars_to_string!(xml));
        }
    }

    /// Launch a defined domain. If the call succeeds the domain moves
    /// from the defined to the running domains pools. The domain will
    /// be paused only if restoring from managed state created from a
    /// paused domain.  For more control, see `create_with_flags()`.
    pub fn create(&self) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainCreate(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(ret as u32);
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
                return Err(Error::last_error());
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
                return Err(Error::last_error());
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
