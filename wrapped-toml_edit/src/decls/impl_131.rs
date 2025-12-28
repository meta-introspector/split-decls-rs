macro_rules! deps {
    () => {
        Key!();
    };
}

macro_rules! impl_131 {
    () => {
        deps!();
        impl std :: hash :: Hash for Key { fn hash < H : std :: hash :: Hasher > (& self , state : & mut H) { self . get () . hash (state) ; } }
    };
}

impl_131!();