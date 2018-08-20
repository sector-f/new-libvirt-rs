bitflags! {
    pub struct SecretXMLFlags: u32 {
        const VIR_SECRET_XML_INACTIVE = 1;
    }
}

bitflags! {
    pub struct SecretUsageType: i32 {
        const VIR_SECRET_USAGE_TYPE_NONE = 0;
        const VIR_SECRET_USAGE_TYPE_VOLUME = 1;
        const VIR_SECRET_USAGE_TYPE_CEPH = 2;
        const VIR_SECRET_USAGE_TYPE_ISCSI = 4;
        const VIR_SECRET_USAGE_TYPE_TLS = 8;
        const VIR_SECRET_USAGE_TYPE_LAST = 16;
    }
}
