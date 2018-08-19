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
        const PAUSED = 1;
        const AUTODESTROY = 2;
        const BYPASS_CACHE = 4;
        const FORCE_BOOT = 8;
        const VALIDATE = 16;
    }
}

bitflags! {
    pub struct XmlFlags: u32 {
        const SECURE = 1;
        const INACTIVE = 2;
        const UPDATE_CPU = 4;
        const MIGRATABLE = 8;
    }
}

bitflags! {
    pub struct DomainDefineFlags: u32 {
        const VALIDATE = 1;
    }
}

bitflags! {
    pub struct DomainDestroyFlags: u32 {
        /// Default behavior - could lead to data loss!!
        const DEFAULT = 0;
        /// only SIGTERM, no SIGKILL
        const GRACEFUL = 1;
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

bitflags! {
    pub struct DomainMemoryModFlags: u32 {
        const CURRENT = 0;
        const LIVE = 1;
        const CONFIG = 2;
        const MAXIMUM = 4;
    }
}

bitflags! {
    pub struct DomainModificationImpact: u32 {
        const CURRENT = 0;
        const LIVE = 	1;
        const CONFIG = 2;
    }
}

bitflags! {
    pub struct DomainVcpuFlags: u32 {
        const CURRENT = 0;
        const LIVE = 1;
        const CONFIG = 2;
        const MAXIMUM = 4;
        const GUEST = 8;
        const HOTPLUGGABLE = 16;
    }
}

bitflags! {
    pub struct DomainSaveRestoreFlags: u32 {
        /// Avoid file system cache pollution
        const BYPASS_CACHE = 1;
        /// Favor running over paused
        const RUNNING = 2;
        /// Favor paused over running
        const PAUSED = 4;
    }
}
