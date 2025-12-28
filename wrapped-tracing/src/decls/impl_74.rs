macro_rules! deps {
    () => {
        Inner!();
    };
}

macro_rules! impl_74 {
    () => {
        deps!();
        impl Hash for Inner { fn hash < H : Hasher > (& self , state : & mut H) { self . id . hash (state) ; } }
    };
}

impl_74!();