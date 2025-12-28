macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_499 {
    () => {
        deps!();
        impl < CTX > HashStable < CTX > for NonZero < usize > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { self . get () . hash_stable (ctx , hasher) } }
    };
}

impl_499!()