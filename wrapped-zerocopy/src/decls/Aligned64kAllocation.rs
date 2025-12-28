macro_rules! Aligned64kAllocation {
    () => {
        # [cfg (__ZEROCOPY_INTERNAL_USE_ONLY_NIGHTLY_FEATURES_IN_TESTS)] # [cfg (not (target_pointer_width = "16"))] # [repr (C , align (65536))] struct Aligned64kAllocation ([u8 ; _64K]) ;
    };
}

Aligned64kAllocation!()