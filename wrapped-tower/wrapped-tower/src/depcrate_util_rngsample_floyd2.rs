// Generated macro for sample_floyd2 (function)
macro_rules! Depcrate_util_rngsample_floyd2 {
() => {
// Module: crate::util::rng
// Provides: {"sample_floyd2"}
// Dependencies: {}
# [doc = " A sampler modified from the Rand implementation for use internally for the balance middleware."] # [doc = ""] # [doc = " It's an implementation of Floyd's combination algorithm with amount fixed at 2. This uses no allocated"] # [doc = " memory and finishes in constant time (only 2 random calls)."] # [doc = ""] # [doc = " ref: This was borrowed and modified from the following Rand implementation"] # [doc = " https://github.com/rust-random/rand/blob/b73640705d6714509f8ceccc49e8df996fa19f51/src/seq/index.rs#L375-L411"] # [cfg (feature = "balance")] pub (crate) fn sample_floyd2 < R : Rng > (rng : & mut R , length : u64) -> [u64 ; 2] { debug_assert ! (2 <= length) ; let aidx = rng . next_range (0 .. length - 1) ; let bidx = rng . next_range (0 .. length) ; let aidx = if aidx == bidx { length - 1 } else { aidx } ; [aidx , bidx] }
};
}
