macro_rules! deps {
    () => {
        HashStable!();
        Interned!();
    };
}

macro_rules! impl_245 {
    () => {
        deps!();
        impl < T , CTX > HashStable < CTX > for Interned < '_ , T > where T : HashStable < CTX > , { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . 0 . hash_stable (hcx , hasher) ; } }
    };
}

impl_245!()