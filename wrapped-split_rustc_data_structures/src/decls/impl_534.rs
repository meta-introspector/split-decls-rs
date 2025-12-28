macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_534 {
    () => {
        deps!();
        impl < I : Idx , T , CTX > HashStable < CTX > for IndexVec < I , T > where T : HashStable < CTX > , { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; for v in & self . raw { v . hash_stable (ctx , hasher) ; } } }
    };
}

impl_534!()