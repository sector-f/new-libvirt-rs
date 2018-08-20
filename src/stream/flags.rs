bitflags! {
    pub struct StreamEventType: u32 {
        const READABLE = 1;
        const WRITABLE = 2;
        const ERROR = 4;
        const HANGUP = 8;
    }
}

bitflags! {
    pub struct StreamFlags: u32 {
        const NONBLOCK = 1;
    }
}

bitflags! {
    pub struct StreamRecvFlagsValues: u32 {
        const STOP_AT_HOLE = 1;
    }
}
