macro_rules! deps {
    () => {
        FileAstId!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < N > Hash for FileAstId < N > { fn hash < H : Hasher > (& self , hasher : & mut H) { self . raw . hash (hasher) ; } }
    };
}

impl_19!()