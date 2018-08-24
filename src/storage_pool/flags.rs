#[derive(Debug)]
pub enum StoragePoolState {
    Inactive = 0,
    Building = 1,
    Running = 2,
    Degraded = 3,
    Inaccessible = 4,
    Last = 5,
}

impl StoragePoolState {
    pub fn from_int(n: i32) -> Option<Self> {
        match n {
            0 => Some(StoragePoolState::Inactive),
            1 => Some(StoragePoolState::Building),
            2 => Some(StoragePoolState::Running),
            3 => Some(StoragePoolState::Degraded),
            4 => Some(StoragePoolState::Inaccessible),
            5 => Some(StoragePoolState::Last),
            _ => None,
        }
    }
}

bitflags! {
    pub struct StoragePoolBuildFlags: u32 {
        /// Regular build from scratch
        const NEW = 0;
        /// Repair / reinitialize
        const REPAIR = 1;
        /// Extend existing pool
        const RESIZE = 2;
        /// Do not overwrite existing pool
        const NO_OVERWRITE = 4;
        /// Overwrite data
        const OVERWRITE = 8;
    }
}

bitflags! {
    pub struct StoragePoolCreateFlags: u32 {
        /// Create the pool and perform pool build without any flags
        const NORMAL = 0;
        /// Create the pool and perform pool build using the VIR_STORAGE_POOL_BUILD_OVERWRITE flag.  This is mutually exclusive to VIR_STORAGE_POOL_CREATE_WITH_BUILD_NO_OVERWRITE
        const WITH_BUILD = 1;
        /// Create the pool and perform pool build using the VIR_STORAGE_POOL_BUILD_NO_OVERWRITE flag. This is mutually exclusive to VIR_STORAGE_POOL_CREATE_WITH_BUILD_OVERWRITE
        const WITH_BUILD_OVERWRITE = 2;
        const WITH_BUILD_NO_OVERWRITE = 4;
    }
}

/// Change to enum?
bitflags! {
    pub struct StoragePoolDeleteFlags: u32 {
        /// Delete metadata only (fast)
        const NORMAL = 0;
        /// Clear all data to zeros (slow)
        const ZEROED = 1;
    }
}

bitflags! {
    pub struct StoragePoolXmlFlags: u32 {
        /// Dump inactive pool/volume information
        const INACTIVE = 1;
    }
}
