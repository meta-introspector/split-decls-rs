macro_rules! deps {
    () => {
        HashStable!();
        UnordBag!();
    };
}

macro_rules! impl_669 {
    () => {
        deps!();
        impl < HCX , V : Hash + Eq + HashStable < HCX > > HashStable < HCX > for UnordBag < V > { # [inline] fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { hash_iter_order_independent (self . inner . iter () , hcx , hasher) ; } }
    };
}

impl_669!();