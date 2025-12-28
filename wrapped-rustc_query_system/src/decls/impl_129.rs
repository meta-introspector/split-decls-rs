macro_rules! deps {
    () => {
        StableHashingContext!();
    };
}

macro_rules! impl_129 {
    () => {
        deps!();
        impl < 'tcx > HashStable < StableHashingContext < 'tcx > > for rustc_feature :: EnabledLangFeature { fn hash_stable (& self , hcx : & mut StableHashingContext < 'tcx > , hasher : & mut StableHasher) { let rustc_feature :: EnabledLangFeature { gate_name , attr_sp , stable_since } = self ; gate_name . hash_stable (hcx , hasher) ; attr_sp . hash_stable (hcx , hasher) ; stable_since . hash_stable (hcx , hasher) ; } }
    };
}

impl_129!()