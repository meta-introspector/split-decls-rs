macro_rules! deps {
    () => {
        UnionField!();
    };
}

macro_rules! impl_330 {
    () => {
        deps!();
        impl < T > :: core :: hash :: Hash for UnionField < T > { fn hash < H : :: core :: hash :: Hasher > (& self , _state : & mut H) { } }
    };
}

impl_330!();