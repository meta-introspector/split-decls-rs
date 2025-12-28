macro_rules! _64K {
    () => {
        # [cfg (__ZEROCOPY_INTERNAL_USE_ONLY_NIGHTLY_FEATURES_IN_TESTS)] # [cfg (not (target_pointer_width = "16"))] const _64K : usize = 1 << 16 ;
    };
}

_64K!();