macro_rules! deps {
    () => {
        HSTRING!();
    };
}

macro_rules! impl_37 {
    () => {
        deps!();
        impl core :: hash :: Hash for HSTRING { fn hash < H : core :: hash :: Hasher > (& self , hasher : & mut H) { self . deref () . hash (hasher) } }
    };
}

impl_37!();