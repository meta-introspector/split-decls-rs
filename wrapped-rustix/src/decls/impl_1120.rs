macro_rules! deps {
    () => {
        WaitIdStatus!();
    };
}

macro_rules! impl_1120 {
    () => {
        deps!();
        # [cfg (linux_raw)] unsafe impl Sync for WaitIdStatus { }
    };
}

impl_1120!()