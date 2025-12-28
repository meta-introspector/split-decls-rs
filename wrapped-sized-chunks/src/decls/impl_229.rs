macro_rules! deps {
    () => {
        RingBuffer!();
    };
}

macro_rules! impl_229 {
    () => {
        deps!();
        impl < A : Hash , const N : usize > Hash for RingBuffer < A , N > { # [inline] fn hash < H : Hasher > (& self , hasher : & mut H) { for item in self { item . hash (hasher) } } }
    };
}

impl_229!();