macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_503 {
    () => {
        deps!();
        impl < T1 : HashStable < CTX > , CTX > HashStable < CTX > for (T1 ,) { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { let (ref _0 ,) = * self ; _0 . hash_stable (ctx , hasher) ; } }
    };
}

impl_503!();