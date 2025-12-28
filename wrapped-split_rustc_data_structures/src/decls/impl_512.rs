macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_512 {
    () => {
        deps!();
        impl < T : HashStable < CTX > , CTX > HashStable < CTX > for Vec < T > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self [..] . hash_stable (ctx , hasher) ; } }
    };
}

impl_512!()