macro_rules! deps {
    () => {
        SockAddr!();
    };
}

macro_rules! impl_15 {
    () => {
        deps!();
        impl Eq for SockAddr { }
    };
}

impl_15!()