macro_rules! deps {
    () => {
        Aligned64kAllocation!();
    };
}

macro_rules! ALIGNED_64K_ALLOCATION {
    () => {
        deps!();
        # [doc = " A pointer to an aligned allocation of size 2^16."] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " `ALIGNED_64K_ALLOCATION` is guaranteed to point to the entirety of an"] # [doc = " allocation with size and alignment 2^16, and to have valid provenance."] # [cfg (__ZEROCOPY_INTERNAL_USE_ONLY_NIGHTLY_FEATURES_IN_TESTS)] # [cfg (not (target_pointer_width = "16"))] pub const ALIGNED_64K_ALLOCATION : NonNull < [u8] > = { const REF : & Aligned64kAllocation = & Aligned64kAllocation ([0 ; _64K]) ; let ptr : * const Aligned64kAllocation = REF ; let ptr : * const [u8] = ptr :: slice_from_raw_parts (ptr . cast () , _64K) ; # [allow (clippy :: as_conversions)] unsafe { NonNull :: new_unchecked (ptr as * mut _) } } ;
    };
}

ALIGNED_64K_ALLOCATION!();