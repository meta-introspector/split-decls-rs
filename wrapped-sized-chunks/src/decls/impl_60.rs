macro_rules! deps {
    () => {
        Chunk!();
    };
}

macro_rules! impl_60 {
    () => {
        deps!();
        impl < A , const N : usize > Hash for Chunk < A , N > where A : Hash , { fn hash < H > (& self , hasher : & mut H) where H : Hasher , { for item in self { item . hash (hasher) } } }
    };
}

impl_60!();