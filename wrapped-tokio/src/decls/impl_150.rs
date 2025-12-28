macro_rules! deps {
    () => {
        ToSocketAddrs!();
    };
}

macro_rules! impl_150 {
    () => {
        deps!();
        impl ToSocketAddrs for SocketAddr { }
    };
}

impl_150!();