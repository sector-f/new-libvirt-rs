bitflags! {
    pub struct BuildFlags: u32 {
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
    pub struct CreateFlags: u32 {
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
    pub struct DeleteFlags: u32 {
        /// Delete metadata only (fast)
        const NORMAL = 0;
        /// Clear all data to zeros (slow)
        const ZEROED = 1;
    }
}
