macro_rules! deps {
    () => {
        ArrayVec!();
        Array!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        impl < A : Array > Hash for ArrayVec < A > where A :: Item : Hash , { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . as_slice () . hash (state) } }
    };
}

impl_57!();