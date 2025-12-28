macro_rules! deps {
    () => {
        Stability!();
    };
}

macro_rules! impl_556 {
    () => {
        deps!();
        impl < CTX > HashStable < CTX > for Stability { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { std :: mem :: discriminant (self) . hash_stable (hcx , hasher) ; match self { Stability :: Stable => { } Stability :: Unstable (nightly_feature) => { nightly_feature . hash_stable (hcx , hasher) ; } Stability :: Forbidden { reason } => { reason . hash_stable (hcx , hasher) ; } } } }
    };
}

impl_556!()