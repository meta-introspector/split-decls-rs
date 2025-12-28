macro_rules! deps {
    () => {
        ToSocketAddrs!();
    };
}

macro_rules! impl_152 {
    () => {
        deps!();
        impl ToSocketAddrs for SocketAddrV4 { }
    };
}

impl_152!();