macro_rules! deps {
    () => {
        Index!();
    };
}

macro_rules! impl_39 {
    () => {
        deps!();
        impl Hash for Index { fn hash < H : Hasher > (& self , state : & mut H) { self . 0 . hash (state) } }
    };
}

impl_39!()