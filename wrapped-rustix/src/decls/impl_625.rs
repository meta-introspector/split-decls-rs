macro_rules! deps {
    () => {
        SocketAddrAny!();
    };
}

macro_rules! impl_625 {
    () => {
        deps!();
        impl Eq for SocketAddrAny { }
    };
}

impl_625!()