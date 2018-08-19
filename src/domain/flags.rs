bitflags! {
    pub struct GetAllDomainStatsFlags: u32 {
        const ACTIVE = 1;
        const INACTIVE = 2;
        const PERSISTENT = 4;
        const TRANSIENT = 8;
        const RUNNING = 16;
        const PAUSED = 32;
        const SHUTOFF = 64;
        const OTHER = 128;
        /// report statistics that can be obtained immediately without any blocking
        const NOWAIT = 536870912;
        /// include backing chain for block stats
        const BACKING = 1073741824;
        /// enforce requested stats
        const ENFORCE_STATS = 2147483648;
    }
}

bitflags! {
    pub struct ListAllDomainsFlags: u32 {
        const ACTIVE = 1;
        const INACTIVE = 2;
        const PERSISTENT = 4;
        const TRANSIENT = 8;
        const RUNNING = 16;
        const PAUSED = 32;
        const SHUTOFF = 64;
        const OTHER = 128;
        const MANAGED_SAVE = 256;
        const NO_MANAGED_SAVE = 512;
        const AUTOSTART = 1024;
        const NO_AUTOSTART = 2048;
        const HAS_SNAPSHOT = 4096;
        const NO_SNAPSHOT = 8192;
    }
}

bitflags! {
    pub struct DomainCreateFlags: u32 {
        const NONE = 0;
        const START_PAUSED = 1;
        const START_AUTODESTROY = 2;
        const START_BYPASS_CACHE = 4;
        const START_FORCE_BOOT = 8;
        const START_VALIDATE = 16;
    }
}

bitflags! {
    pub struct XmlFlags: u32 {
        const XML_SECURE = 1;
        const XML_INACTIVE = 2;
        const XML_UPDATE_CPU = 4;
        const XML_MIGRATABLE = 8;
    }
}

bitflags! {
    pub struct DomainDefineFlags: u32 {
        const DEFINE_VALIDATE = 1;
    }
}

bitflags! {
    pub struct DomainDestroyFlags: u32 {
        /// Default behavior - could lead to data loss!!
        const DESTROY_DEFAULT = 0;
        /// only SIGTERM, no SIGKILL
        const DESTROY_GRACEFUL = 1;
    }
}

bitflags! {
    pub struct RebootFlags: u32 {
        /// Hypervisor choice
        const DEFAULT = 0;
        /// Send ACPI event
        const ACPI_POWER_BTN = 1;
        /// Use guest agent
        const GUEST_AGENT = 2;
        /// Use initctl
        const INITCTL = 4;
        /// Send a signal
        const SIGNAL = 8;
        /// Use paravirt guest control
        const PARAVIRT = 16;
    }
}
