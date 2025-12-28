macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_537 {
    () => {
        deps!();
        impl < T , CTX > HashStable < CTX > for bit_set :: FiniteBitSet < T > where T : HashStable < CTX > + bit_set :: FiniteBitSetTy , { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . 0 . hash_stable (hcx , hasher) ; } }
    };
}

impl_537!()