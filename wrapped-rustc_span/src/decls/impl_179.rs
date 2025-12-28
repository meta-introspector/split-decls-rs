macro_rules! deps {
    () => {
        Ident!();
    };
}

macro_rules! impl_179 {
    () => {
        deps!();
        impl Hash for Ident { fn hash < H : Hasher > (& self , state : & mut H) { self . name . hash (state) ; self . span . ctxt () . hash (state) ; } }
    };
}

impl_179!();