macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_511 {
    () => {
        deps!();
        impl < CTX > HashStable < CTX > for [u8] { fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . len () . hash_stable (ctx , hasher) ; hasher . write (self) ; } }
    };
}

impl_511!();