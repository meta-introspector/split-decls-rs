macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_531 {
    () => {
        deps!();
        impl < T , CTX > HashStable < CTX > for :: std :: mem :: Discriminant < T > { # [inline] fn hash_stable (& self , _ : & mut CTX , hasher : & mut StableHasher) { :: std :: hash :: Hash :: hash (self , hasher) ; } }
    };
}

impl_531!();