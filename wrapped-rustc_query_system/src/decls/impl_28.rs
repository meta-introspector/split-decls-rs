macro_rules! deps {
    () => {
        WorkProductId!();
    };
}

macro_rules! impl_28 {
    () => {
        deps!();
        impl < HCX > HashStable < HCX > for WorkProductId { # [inline] fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { self . hash . hash_stable (hcx , hasher) } }
    };
}

impl_28!()