macro_rules! impl_251 {
    () => {
        impl Hash for Index { fn hash < H : Hasher > (& self , state : & mut H) { self . index . hash (state) ; } }
    };
}

impl_251!()