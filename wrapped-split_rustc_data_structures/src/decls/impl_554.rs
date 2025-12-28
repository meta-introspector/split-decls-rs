macro_rules! deps {
    () => {
        HashStable!();
        Steal!();
    };
}

macro_rules! impl_554 {
    () => {
        deps!();
        impl < CTX , T : HashStable < CTX > > HashStable < CTX > for Steal < T > { fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self . borrow () . hash_stable (hcx , hasher) ; } }
    };
}

impl_554!()