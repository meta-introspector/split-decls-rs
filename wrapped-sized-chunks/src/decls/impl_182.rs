macro_rules! deps {
    () => {
        Slice!();
    };
}

macro_rules! impl_182 {
    () => {
        deps!();
        impl < 'a , A : Hash + 'a , const N : usize > Hash for Slice < 'a , A , N > { # [inline] fn hash < H : Hasher > (& self , hasher : & mut H) { for item in self { item . hash (hasher) } } }
    };
}

impl_182!()