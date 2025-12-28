macro_rules! deps {
    () => {
        Handle!();
    };
}

macro_rules! impl_5 {
    () => {
        deps!();
        impl AsRawFd for crate :: Handle { fn as_raw_fd (& self) -> RawFd { self . 0 . file . as_ref () . take () . unwrap () . as_raw_fd () } }
    };
}

impl_5!()