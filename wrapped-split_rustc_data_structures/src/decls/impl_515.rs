macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_515 {
    () => {
        deps!();
        impl < A , const N : usize , CTX > HashStable < CTX > for SmallVec < [A ; N] > where A : HashStable < CTX > , { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self [..] . hash_stable (ctx , hasher) ; } }
    };
}

impl_515!();