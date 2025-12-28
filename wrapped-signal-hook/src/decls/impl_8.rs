macro_rules! deps {
    () => {
        SelfPipeWrite!();
        WakeMethod!();
    };
}

macro_rules! impl_8 {
    () => {
        deps!();
        impl < W : AsRawFd + Debug + Send + Sync > SelfPipeWrite for W { fn wake_readers (& self) { pipe :: wake (self . as_raw_fd () , WakeMethod :: Send) ; } }
    };
}

impl_8!();