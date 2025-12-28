macro_rules! deps {
    () => {
        Field!();
    };
}

macro_rules! impl_172 {
    () => {
        deps!();
        impl Hash for Field { fn hash < H > (& self , state : & mut H) where H : Hasher , { self . callsite () . hash (state) ; self . i . hash (state) ; } }
    };
}

impl_172!()