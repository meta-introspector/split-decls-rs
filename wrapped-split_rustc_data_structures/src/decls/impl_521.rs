macro_rules! deps {
    () => {
        HashStable!();
    };
}

macro_rules! impl_521 {
    () => {
        deps!();
        impl < CTX > HashStable < CTX > for String { # [inline] fn hash_stable (& self , hcx : & mut CTX , hasher : & mut StableHasher) { self [..] . hash_stable (hcx , hasher) ; } }
    };
}

impl_521!()