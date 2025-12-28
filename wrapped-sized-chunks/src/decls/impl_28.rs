macro_rules! deps {
    () => {
        InlineArray!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < A , T > Hash for InlineArray < A , T > where A : Hash , { fn hash < H > (& self , hasher : & mut H) where H : Hasher , { for item in self { item . hash (hasher) } } }
    };
}

impl_28!();