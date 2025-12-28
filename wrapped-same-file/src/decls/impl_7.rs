macro_rules! deps {
    () => {
        Handle!();
    };
}

macro_rules! impl_7 {
    () => {
        deps!();
        impl Hash for Handle { fn hash < H : Hasher > (& self , state : & mut H) { self . dev . hash (state) ; self . ino . hash (state) ; } }
    };
}

impl_7!();