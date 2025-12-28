macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_506 {
    () => {
        deps!();
        impl < T1 , T2 , T3 , CTX > HashStable < CTX > for (T1 , T2 , T3) where T1 : HashStable < CTX > , T2 : HashStable < CTX > , T3 : HashStable < CTX > , { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { let (ref _0 , ref _1 , ref _2) = * self ; _0 . hash_stable (ctx , hasher) ; _1 . hash_stable (ctx , hasher) ; _2 . hash_stable (ctx , hasher) ; } }
    };
}

impl_506!()