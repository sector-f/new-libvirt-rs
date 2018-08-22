bitflags! {
    pub struct StorageVolCreateFlags: u32 {
        const PREALLOC_METADATA = 1;
        const REFLINK = 2;
    }
}

bitflags! {
    pub struct StorageVolDeleteFlags: u32 {
        const NORMAL = 0;
        const ZEROED = 1;
        const WITH_SNAPSHOTS = 2;
    }
}

bitflags! {
    pub struct StorageVolDownloadFlags: u32 {
        const SPARSE_STREAM = 1;
    }
}

bitflags! {
    pub struct StorageVolInfoFlags: u32 {
        const USE_ALLOCATION = 0;
        const GET_PHYSICAL = 1;
    }
}

bitflags! {
    pub struct StorageVolResizeFlags: u32 {
        const ALLOCATE = 1;
        const DELTA = 2;
        const SHRINK = 4;
    }
}

pub enum StorageVolType {
    File = 0,
    Block = 1,
    Dir = 2,
    Network = 3,
    NetDir = 4,
    Ploop = 5,
    Last = 6,
}

bitflags! {
    pub struct StorageVolUploadFlags: u32 {
        const SPARSE_STREAM = 1;
    }
}

pub enum StorageVolWipeAlgorithm {
    /// 1-pass, all zeroes
    Zero = 0,
    /// 4-pass NNSA Policy Letter NAP-14.1-C (XVI-8)
    Nnsa = 1,
    /// 4-pass DoD 5220.22-M section 8-306 procedure
    Dod = 2,
    /// 9-pass method recommended by the German Center of Security in Information Technologies
    Bsi = 3,
    /// The canonical 35-pass sequence
    Gutmann = 4,
    /// 7-pass method described by Bruce Schneier in "Applied Cryptography" (1996)
    Schneier = 5,
    /// 7-pass random
    Pfitzner7 = 6,
    /// 33-pass random
    Pfitzner33 = 7,
    /// 1-pass random
    Random = 8,
    /// 1-pass, trim all data on the volume by using TRIM or DISCARD
    Trim = 9,
    /// NB: this enum value will increase over time as new algorithms are added to the libvirt API. It reflects the last algorithm supported by this version of the libvirt API.
    Last = 10,
}
