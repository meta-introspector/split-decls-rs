macro_rules! deps {
    () => {
        WakeFd!();
    };
}

macro_rules! impl_71 {
    () => {
        deps!();
        impl AsRawFd for WakeFd { fn as_raw_fd (& self) -> RawFd { self . fd } }
    };
}

impl_71!();