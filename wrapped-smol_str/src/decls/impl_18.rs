macro_rules! deps {
    () => {
        SmolStr!();
    };
}

macro_rules! impl_18 {
    () => {
        deps!();
        impl hash :: Hash for SmolStr { fn hash < H : hash :: Hasher > (& self , hasher : & mut H) { self . as_str () . hash (hasher) ; } }
    };
}

impl_18!();