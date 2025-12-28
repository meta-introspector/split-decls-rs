macro_rules! deps {
    () => {
        ToSocketAddrs!();
    };
}

macro_rules! impl_160 {
    () => {
        deps!();
        impl ToSocketAddrs for (Ipv6Addr , u16) { }
    };
}

impl_160!()