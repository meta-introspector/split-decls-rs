macro_rules! deps {
    () => {
        EdgesVec!();
    };
}

macro_rules! impl_34 {
    () => {
        deps!();
        impl Hash for EdgesVec { # [inline] fn hash < H : Hasher > (& self , hasher : & mut H) { Hash :: hash (& self . edges , hasher) } }
    };
}

impl_34!()