extern crate libc;
extern crate libvirt_sys as sys;
use error::Error;

use connect::Connect;
use std::ffi::CStr;
use std::{ptr, slice, mem};
use std::os::raw::c_int;

pub mod flags;
use domain::flags::*;

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
    pub fn get_xml_desc(&self, flags: XmlFlags) -> Result<String, Error> {
        unsafe {
            let xml = sys::virDomainGetXMLDesc(self.as_ptr(), flags.bits());
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
    pub fn create(&self) -> Result<(), Error> {
        unsafe {
            let ret = sys::virDomainCreate(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    /// Launch a defined domain. If the call succeeds the domain moves
    /// from the defined to the running domains pools.
    pub fn create_with_flags(&self, flags: DomainCreateFlags) -> Result<(), Error> {
        unsafe {
            let res = sys::virDomainCreateWithFlags(self.as_ptr(), flags.bits());
            if res == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    /// Launch a new guest domain, based on an XML description similar
    /// to the one returned by `get_xml_desc()`.
    ///
    /// This function may require privileged access to the hypervisor.
    ///
    /// The domain is not persistent, so its definition will disappear
    /// when it is destroyed, or if the host is restarted (see
    /// `define_xml()` to define persistent domains).
    pub fn create_xml(conn: &Connect, xml: &str, flags: DomainCreateFlags) -> Result<Domain, Error> {
        unsafe {
            let ptr = sys::virDomainCreateXML(conn.as_ptr(), string_to_c_chars!(xml), flags.bits());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Domain::new(ptr));
        }
    }

    /// Define a domain, but does not start it.
    ///
    /// This definition is persistent, until explicitly undefined with
    /// `undefine()`. A previous definition for this domain would be
    /// overridden if it already exists.
    ///
    /// # Note:
    ///
    /// Some hypervisors may prevent this operation if there is a
    /// current block copy operation on a transient domain with the
    /// same id as the domain being defined.
    pub fn define_xml(conn: &Connect, xml: &str) -> Result<Domain, Error> {
        unsafe {
            let ptr = sys::virDomainDefineXML(conn.as_ptr(), string_to_c_chars!(xml));
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Domain::new(ptr));
        }
    }

    /// Define a domain, but does not start it.
    ///
    /// This definition is persistent, until explicitly undefined with
    /// `undefine()`. A previous definition for this domain would be
    /// overridden if it already exists.
    ///
    /// # Note:
    ///
    /// Some hypervisors may prevent this operation if there is a
    /// current block copy operation on a transient domain with the
    /// same id as the domain being defined.
    pub fn define_xml_flags(conn: &Connect,
                            xml: &str,
                            flags: DomainDefineFlags)
                            -> Result<Domain, Error> {
        unsafe {
            let ptr = sys::virDomainDefineXMLFlags(conn.as_ptr(), string_to_c_chars!(xml), flags.bits());
            if ptr.is_null() {
                return Err(Error::last_error());
            }
            return Ok(Domain::new(ptr));
        }
    }

    /// Destroy the domain. The running instance is shutdown if not
    /// down already and all resources used by it are given back to
    /// the hypervisor. This does not free the associated virDomainPtr
    /// object. This function may require privileged access.
    pub fn destroy(&self) -> Result<(), Error> {
        unsafe {
            if sys::virDomainDestroy(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    /// Destroy the domain. The running instance is shutdown if not
    /// down already and all resources used by it are given back to
    /// the hypervisor. This does not free the associated virDomainPtr
    /// object. This function may require privileged access.
    pub fn destroy_flags(&self, flags: DomainDestroyFlags) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainDestroyFlags(self.as_ptr(), flags.bits());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(ret as u32);
        }
    }

    /// Shutdown a domain
    ///
    /// The domain object is still usable thereafter, but the domain
    /// OS is being stopped. Note that the guest OS may ignore the
    /// request. Additionally, the hypervisor may check and support
    /// the domain 'on_poweroff' XML setting resulting in a domain
    /// that reboots instead of shutting down. For guests that react
    /// to a shutdown request, the differences from `destroy()` are
    /// that the guests disk storage will be in a stable state rather
    /// than having the (virtual) power cord pulled, and this command
    /// returns as soon as the shutdown request is issued rather than
    /// blocking until the guest is no longer running.
    pub fn shutdown(&self) -> Result<(), Error> {
        unsafe {
            let ret = sys::virDomainShutdown(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }


    /// Reboot a domain. Passing `None` will have the same effect as
    /// passing `Some(RebootFlags::DEFAULT)`
    ///
    /// The domain object is still usable thereafter.
    pub fn reboot(&self, flags: Option<RebootFlags>) -> Result<(), Error> {
        let flags = flags.and_then(|f| Some(f.bits())).unwrap_or(0);
        unsafe {
            if sys::virDomainReboot(self.as_ptr(), flags) == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    /// Suspend a domain.
    ///
    /// Suspends an active domain, the process is frozen without
    /// further access to CPU resources and I/O but the memory used by
    /// the domain at the hypervisor level will stay allocated. Use
    /// `resume` to reactivate the domain.  This function may
    /// require privileged access.  Moreover, suspend may not be
    /// supported if domain is in some special state like
    /// VIR_DOMAIN_PMSUSPENDED.
    pub fn suspend(&self) -> Result<(), Error> {
        unsafe {
            let ret = sys::virDomainSuspend(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    /// Resume a suspended domain.
    ///
    /// the process is restarted from the state where it was frozen by
    /// calling `suspend()`. This function may require privileged
    /// access Moreover, resume may not be supported if domain is in
    /// some special state like VIR_DOMAIN_PMSUSPENDED.
    pub fn resume(&self) -> Result<u32, Error> {
        unsafe {
            let ret = sys::virDomainResume(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(ret as u32);
        }
    }

    /// Determine if the domain is currently running.
    pub fn is_active(&self) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virDomainIsActive(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(ret == 1);
        }
    }

    /// Undefine a domain.
    ///
    /// If the domain is running, it's converted to transient domain,
    /// without stopping it. If the domain is inactive, the domain
    /// configuration is removed.
    pub fn undefine(&self) -> Result<(), Error> {
        unsafe {
            if sys::virDomainUndefine(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    /// Free the domain object.
    ///
    /// The running instance is kept alive. The data structure is
    /// freed and should not be used thereafter.
    pub fn free(&mut self) -> Result<(), Error> {
        unsafe {
            if sys::virDomainFree(self.as_ptr()) == -1 {
                return Err(Error::last_error());
            }
            self.ptr = None;
            return Ok(());
        }
    }

    pub fn is_updated(&self) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virDomainIsUpdated(self.as_ptr());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(ret == 1);
        }
    }

    pub fn get_autostart(&self) -> Result<bool, Error> {
        unsafe {
            let autostart: *mut c_int = 0 as *mut i32;
            let ret = sys::virDomainGetAutostart(self.as_ptr(), autostart);
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(true);
        }
    }

    pub fn set_autostart(&self, autostart: bool) -> Result<bool, Error> {
        unsafe {
            let ret = sys::virDomainSetAutostart(self.as_ptr(), autostart as libc::c_int);
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(ret == 1);
        }
    }

    pub fn set_max_memory(&self, memory: u64) -> Result<(), Error> {
        unsafe {
            let ret = sys::virDomainSetMaxMemory(self.as_ptr(), memory as libc::c_ulong);
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn get_max_memory(&self) -> Result<u64, Error> {
        unsafe {
            let ret = sys::virDomainGetMaxMemory(self.as_ptr());
            if ret == 0 {
                return Err(Error::last_error());
            }
            return Ok(ret as u64);
        }
    }

    pub fn get_max_vcpus(&self) -> Result<u64, Error> {
        unsafe {
            let ret = sys::virDomainGetMaxVcpus(self.as_ptr());
            if ret == 0 {
                return Err(Error::last_error());
            }
            return Ok(ret as u64);
        }
    }

    pub fn set_memory(&self, memory: u64) -> Result<(), Error> {
        unsafe {
            let ret = sys::virDomainSetMemory(self.as_ptr(), memory as libc::c_ulong);
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
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

    pub fn set_memory_flags(&self, memory: u64, flags: DomainMemoryModFlags) -> Result<(), Error> {
        unsafe {
            let ret = sys::virDomainSetMemoryFlags(self.as_ptr(), memory as libc::c_ulong, flags.bits());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }
    pub fn set_memory_stats_period(&self, period: i32, flags: DomainMemoryModFlags) -> Result<(), Error> {
        unsafe {
            let ret = sys::virDomainSetMemoryStatsPeriod(self.as_ptr(), period as libc::c_int, flags.bits());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn set_vcpus(&self, vcpus: u32) -> Result<(), Error> {
        unsafe {
            let ret = sys::virDomainSetVcpus(self.as_ptr(), vcpus as libc::c_uint);
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn set_vcpus_flags(&self, vcpus: u32, flags: DomainVcpuFlags) -> Result<(), Error> {
        unsafe {
            let ret = sys::virDomainSetVcpusFlags(self.as_ptr(), vcpus as libc::c_uint, flags.bits());
            if ret == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn domain_restore(conn: &Connect, path: &str) -> Result<(), Error> {
        unsafe {
            if sys::virDomainRestore(conn.as_ptr(), string_to_c_chars!(path)) == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
        }
    }

    pub fn domain_restore_flags(conn: &Connect, path: &str, dxml: Option<&str>, flags: DomainSaveRestoreFlags) -> Result<(), Error> {
        unsafe {
            let xml = match dxml {
                Some(c) => string_to_c_chars!(c),
                None => ptr::null(),
            };

            if sys::virDomainRestoreFlags(conn.as_ptr(), string_to_c_chars!(path), xml, flags.bits()) == -1 {
                return Err(Error::last_error());
            }
            return Ok(());
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
