macro_rules! deps {
    () => {
        Unalign!();
        Unaligned!();
    };
}

macro_rules! impl_419 {
    () => {
        deps!();
        impl < T : Unaligned + Hash > Hash for Unalign < T > { # [inline (always)] fn hash < H > (& self , state : & mut H) where H : Hasher , { self . deref () . hash (state) ; } }
    };
}

impl_419!();