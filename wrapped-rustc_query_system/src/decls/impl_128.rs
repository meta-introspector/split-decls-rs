macro_rules! deps {
    () => {
        StableHashingContext!();
    };
}

macro_rules! impl_128 {
    () => {
        deps!();
        impl < 'tcx > HashStable < StableHashingContext < 'tcx > > for rustc_feature :: Features { fn hash_stable (& self , hcx : & mut StableHashingContext < 'tcx > , hasher : & mut StableHasher) { self . enabled_lang_features () . hash_stable (hcx , hasher) ; self . enabled_lib_features () . hash_stable (hcx , hasher) ; } }
    };
}

impl_128!()