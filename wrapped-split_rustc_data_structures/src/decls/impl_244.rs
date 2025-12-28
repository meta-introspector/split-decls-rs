macro_rules! deps {
    () => {
        Interned!();
    };
}

macro_rules! impl_244 {
    () => {
        deps!();
        impl < 'a , T > Hash for Interned < 'a , T > where T : Hash , { # [inline] fn hash < H : Hasher > (& self , s : & mut H) { ptr :: hash (self . 0 , s) } }
    };
}

impl_244!()