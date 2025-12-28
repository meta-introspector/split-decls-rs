macro_rules! deps {
    () => {
        LintId!();
    };
}

macro_rules! impl_22 {
    () => {
        deps!();
        impl < HCX > HashStable < HCX > for LintId { # [inline] fn hash_stable (& self , hcx : & mut HCX , hasher : & mut StableHasher) { self . lint_name_raw () . hash_stable (hcx , hasher) ; } }
    };
}

impl_22!()