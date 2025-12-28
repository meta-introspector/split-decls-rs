macro_rules! deps {
    () => {
        AsAddress!();
    };
}

macro_rules! impl_76 {
    () => {
        deps!();
        impl < T : ? Sized > AsAddress for * const T { # [inline (always)] fn addr (self) -> usize { # [allow (clippy :: as_conversions)] # [cfg_attr (__ZEROCOPY_INTERNAL_USE_ONLY_NIGHTLY_FEATURES_IN_TESTS , allow (lossy_provenance_casts))] return self . cast :: < () > () as usize ; } }
    };
}

impl_76!()