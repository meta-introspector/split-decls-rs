macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_494 {
    () => {
        deps!();
        impl < CTX > HashStable < CTX > for Hash128 { # [inline] fn hash_stable (& self , _ : & mut CTX , hasher : & mut StableHasher) { self . as_u128 () . hash (hasher) ; } }
    };
}

impl_494!();