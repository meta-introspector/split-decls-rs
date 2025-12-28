macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_532 {
    () => {
        deps!();
        impl < T , CTX > HashStable < CTX > for :: std :: ops :: RangeInclusive < T > where T : HashStable < CTX > , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . start () . hash_stable (ctx , hasher) ; self . end () . hash_stable (ctx , hasher) ; } }
    };
}

impl_532!();