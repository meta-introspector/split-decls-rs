macro_rules! deps {
    () => {
        HashStable!();
        UnordSet!();
    };
}

macro_rules! impl_654 {
    () => {
        deps!();
        impl < HCX , V : Hash + Eq + HashStable < HCX > > HashStable < HCX > for UnordSet < V > { # [inline] fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { hash_iter_order_independent (self . inner . iter () , hcx , hasher) ; } }
    };
}

impl_654!()