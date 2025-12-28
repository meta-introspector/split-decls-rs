macro_rules! deps {
    () => {
        SyncNotSend!();
    };
}

macro_rules! impl_339 {
    () => {
        deps!();
        unsafe impl Sync for SyncNotSend { }
    };
}

impl_339!();