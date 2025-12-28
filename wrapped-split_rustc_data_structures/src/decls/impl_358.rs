macro_rules! deps {
    () => {
        HashStable!();
        Pu128!();
    };
}

macro_rules! impl_358 {
    () => {
        deps!();
        impl < CTX > HashStable < CTX > for Pu128 { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { { self . 0 } . hash_stable (ctx , hasher) } }
    };
}

impl_358!()