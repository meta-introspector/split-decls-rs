macro_rules! deps {
    () => {
        Span!();
    };
}

macro_rules! impl_64 {
    () => {
        deps!();
        impl Hash for Span { fn hash < H : Hasher > (& self , hasher : & mut H) { self . inner . hash (hasher) ; } }
    };
}

impl_64!();