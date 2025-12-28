macro_rules! deps {
    () => {
        ToSocketAddrs!();
    };
}

macro_rules! impl_158 {
    () => {
        deps!();
        impl ToSocketAddrs for (Ipv4Addr , u16) { }
    };
}

impl_158!();