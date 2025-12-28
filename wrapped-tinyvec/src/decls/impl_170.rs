macro_rules! deps {
    () => {
        Array!();
        TinyVec!();
    };
}

macro_rules! impl_170 {
    () => {
        deps!();
        impl < A : Array > Hash for TinyVec < A > where A :: Item : Hash , { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . as_slice () . hash (state) } }
    };
}

impl_170!()