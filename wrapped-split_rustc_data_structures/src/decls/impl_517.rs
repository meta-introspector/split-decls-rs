macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_517 {
    () => {
        deps!();
        impl < T : ? Sized + HashStable < CTX > , CTX > HashStable < CTX > for :: std :: rc :: Rc < T > { # [inline] fn hash_stable (& self , ctx : & mut CTX , hasher : & mut StableHasher) { (* * self) . hash_stable (ctx , hasher) ; } }
    };
}

impl_517!();