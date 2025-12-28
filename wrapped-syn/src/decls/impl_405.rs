macro_rules! deps {
    () => {
        Lifetime!();
    };
}

macro_rules! impl_405 {
    () => {
        deps!();
        impl Hash for Lifetime { fn hash < H : Hasher > (& self , h : & mut H) { self . ident . hash (h) ; } }
    };
}

impl_405!()