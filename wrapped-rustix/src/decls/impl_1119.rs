macro_rules! deps {
    () => {
        WaitIdStatus!();
    };
}

macro_rules! impl_1119 {
    () => {
        deps!();
        # [cfg (linux_raw)] unsafe impl Send for WaitIdStatus { }
    };
}

impl_1119!();