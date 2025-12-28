macro_rules! deps {
    () => {
        StableHashingContext!();
    };
}

macro_rules! impl_130 {
    () => {
        deps!();
        impl < 'tcx > HashStable < StableHashingContext < 'tcx > > for rustc_feature :: EnabledLibFeature { fn hash_stable (& self , hcx : & mut StableHashingContext < 'tcx > , hasher : & mut StableHasher) { let rustc_feature :: EnabledLibFeature { gate_name , attr_sp } = self ; gate_name . hash_stable (hcx , hasher) ; attr_sp . hash_stable (hcx , hasher) ; } }
    };
}

impl_130!();