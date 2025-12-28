macro_rules! deps {
    () => {
        ToSocketAddrs!();
    };
}

macro_rules! impl_154 {
    () => {
        deps!();
        impl ToSocketAddrs for SocketAddrV6 { }
    };
}

impl_154!()