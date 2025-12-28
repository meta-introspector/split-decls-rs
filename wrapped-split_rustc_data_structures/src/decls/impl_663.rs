macro_rules! deps {
    () => {
        UnordMap!();
        HashStable!();
    };
}

macro_rules! impl_663 {
    () => {
        deps!();
        impl < HCX , K : Hash + Eq + HashStable < HCX > , V : HashStable < HCX > > HashStable < HCX > for UnordMap < K , V > { # [inline] fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { hash_iter_order_independent (self . inner . iter () , hcx , hasher) ; } }
    };
}

impl_663!();