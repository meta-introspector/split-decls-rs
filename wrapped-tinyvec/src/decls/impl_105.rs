macro_rules! deps {
    () => {
        SliceVec!();
    };
}

macro_rules! impl_105 {
    () => {
        deps!();
        impl < 's , T > Hash for SliceVec < 's , T > where T : Hash , { # [inline] fn hash < H : Hasher > (& self , state : & mut H) { self . as_slice () . hash (state) } }
    };
}

impl_105!();