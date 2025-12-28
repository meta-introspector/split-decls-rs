macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_502 {
    () => {
        deps!();
        impl < CTX > HashStable < CTX > for :: std :: cmp :: Ordering { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (* self as i8) . hash_stable (ctx , hasher) ; } }
    };
}

impl_502!()