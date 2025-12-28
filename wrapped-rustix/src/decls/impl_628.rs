macro_rules! deps {
    () => {
        SocketAddrAny!();
    };
}

macro_rules! impl_628 {
    () => {
        deps!();
        impl core :: hash :: Hash for SocketAddrAny { fn hash < H : core :: hash :: Hasher > (& self , state : & mut H) { self . bytes () . hash (state) } }
    };
}

impl_628!()