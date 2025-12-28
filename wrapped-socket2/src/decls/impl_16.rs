macro_rules! deps {
    () => {
        SockAddr!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl Hash for SockAddr { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . as_bytes () . hash (state) ; } }
    };
}

impl_16!();