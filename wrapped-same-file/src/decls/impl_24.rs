macro_rules! deps {
    () => {
        Handle!();
    };
}

macro_rules! impl_24 {
    () => {
        deps!();
        impl Hash for Handle { fn hash < H : Hasher > (& self , state : & mut H) { self . key . hash (state) ; } }
    };
}

impl_24!()