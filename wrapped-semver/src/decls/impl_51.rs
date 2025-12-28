macro_rules! deps {
    () => {
        Identifier!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl Hash for Identifier { fn hash < H : Hasher > (& self , hasher : & mut H) { self . as_str () . hash (hasher) ; } }
    };
}

impl_51!()