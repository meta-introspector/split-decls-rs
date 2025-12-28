macro_rules! deps {
    () => {
        Handle!();
    };
}

macro_rules! impl_6 {
    () => {
        deps!();
        impl IntoRawFd for crate :: Handle { fn into_raw_fd (mut self) -> RawFd { self . 0 . file . take () . unwrap () . into_raw_fd () } }
    };
}

impl_6!();