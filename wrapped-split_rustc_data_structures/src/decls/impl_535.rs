macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_535 {
    () => {
        deps!();
        impl < I : Idx , CTX > HashStable < CTX > for DenseBitSet < I > { fn hash_stable (& self , _ctx : & mut CTX , hasher : & mut StableHasher) { :: std :: hash :: Hash :: hash (self , hasher) ; } }
    };
}

impl_535!()