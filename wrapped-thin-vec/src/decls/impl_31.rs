macro_rules! deps {
    () => {
        ThinVec!();
    };
}

macro_rules! impl_31 {
    () => {
        deps!();
        impl < T > Hash for ThinVec < T > where T : Hash , { fn hash < H > (& self , state : & mut H) where H : Hasher , { self [..] . hash (state) ; } }
    };
}

impl_31!();