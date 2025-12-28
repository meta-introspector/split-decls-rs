macro_rules! deps {
    () => {
        AlignOf!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < T > AlignOf < T > { # [inline (never)] # [cfg_attr (all (coverage_nightly , __ZEROCOPY_INTERNAL_USE_ONLY_NIGHTLY_FEATURES_IN_TESTS) , coverage (off))] pub fn into_t (self) -> T { unreachable ! () } }
    };
}

impl_32!()