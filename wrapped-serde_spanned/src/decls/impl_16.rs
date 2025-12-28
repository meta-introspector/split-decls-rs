macro_rules! deps {
    () => {
        Spanned!();
    };
}

macro_rules! impl_16 {
    () => {
        deps!();
        impl < T : Hash > Hash for Spanned < T > { fn hash < H : Hasher > (& self , state : & mut H) { self . value . hash (state) ; } }
    };
}

impl_16!();