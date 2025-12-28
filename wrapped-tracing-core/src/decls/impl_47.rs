macro_rules! deps {
    () => {
        Callsite!();
        Identifier!();
    };
}

macro_rules! impl_47 {
    () => {
        deps!();
        impl Hash for Identifier { fn hash < H > (& self , state : & mut H) where H : Hasher , { (self . 0 as * const dyn Callsite) . hash (state) } }
    };
}

impl_47!()