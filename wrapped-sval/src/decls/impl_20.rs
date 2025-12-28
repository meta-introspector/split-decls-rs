macro_rules! deps {
    () => {
        Label!();
    };
}

macro_rules! impl_20 {
    () => {
        deps!();
        impl < 'a > Hash for Label < 'a > { fn hash < H : Hasher > (& self , state : & mut H) { self . as_str () . hash (state) } }
    };
}

impl_20!()