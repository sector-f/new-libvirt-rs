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
    pub struct ListAllStoragePoolsFlags: u32 {
        const VIR_CONNECT_LIST_STORAGE_POOLS_INACTIVE = 1;
        const VIR_CONNECT_LIST_STORAGE_POOLS_ACTIVE = 2;
        const VIR_CONNECT_LIST_STORAGE_POOLS_PERSISTENT = 4;
        const VIR_CONNECT_LIST_STORAGE_POOLS_TRANSIENT = 8;
        const VIR_CONNECT_LIST_STORAGE_POOLS_AUTOSTART = 16;
        const VIR_CONNECT_LIST_STORAGE_POOLS_NO_AUTOSTART = 32 	;
        const VIR_CONNECT_LIST_STORAGE_POOLS_DIR = 64;
        const VIR_CONNECT_LIST_STORAGE_POOLS_FS = 128;
        const VIR_CONNECT_LIST_STORAGE_POOLS_NETFS = 256;
        const VIR_CONNECT_LIST_STORAGE_POOLS_LOGICAL = 512;
        const VIR_CONNECT_LIST_STORAGE_POOLS_DISK = 1024;
        const VIR_CONNECT_LIST_STORAGE_POOLS_ISCSI = 2048;
        const VIR_CONNECT_LIST_STORAGE_POOLS_SCSI = 4096;
        const VIR_CONNECT_LIST_STORAGE_POOLS_MPATH = 8192;
        const VIR_CONNECT_LIST_STORAGE_POOLS_RBD = 16384;
        const VIR_CONNECT_LIST_STORAGE_POOLS_SHEEPDOG = 32768;
        const VIR_CONNECT_LIST_STORAGE_POOLS_GLUSTER = 65536;
        const VIR_CONNECT_LIST_STORAGE_POOLS_ZFS = 131072;
        const VIR_CONNECT_LIST_STORAGE_POOLS_VSTORAGE = 262144;
    }
}
