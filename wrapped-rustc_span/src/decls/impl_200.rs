macro_rules! deps {
    () => {
        Symbol!();
    };
}

macro_rules! impl_200 {
    () => {
        deps!();
        impl < CTX > HashStable < CTX > for Symbol { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . as_str () . hash_stable (hcx , hasher) ; } }
    };
}

impl_200!();