macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_519 {
    () => {
        deps!();
        impl < CTX > HashStable < CTX > for str { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . as_bytes () . hash_stable (ctx , hasher) ; } }
    };
}

impl_519!();