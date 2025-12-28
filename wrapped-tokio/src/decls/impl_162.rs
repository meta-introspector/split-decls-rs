macro_rules! deps {
    () => {
        ToSocketAddrs!();
    };
}

macro_rules! impl_162 {
    () => {
        deps!();
        impl ToSocketAddrs for & [SocketAddr] { }
    };
}

impl_162!();