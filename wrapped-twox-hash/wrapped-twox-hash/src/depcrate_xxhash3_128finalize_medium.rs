// Generated macro for finalize_medium (function)
macro_rules! Depcrate_xxhash3_128finalize_medium {
() => {
// Module: crate::xxhash3_128
// Provides: {"finalize_medium"}
// Dependencies: {}
# [inline] fn finalize_medium (acc : [u64 ; 2] , input_len : u64 , seed : u64) -> u128 { let low = acc [0] . wrapping_add (acc [1]) ; let high = acc [0] . wrapping_mul (PRIME64_1) . wrapping_add (acc [1] . wrapping_mul (PRIME64_4)) . wrapping_add ((input_len . wrapping_sub (seed)) . wrapping_mul (PRIME64_2)) ; let low = avalanche (low) ; let high = avalanche (high) . wrapping_neg () ; X128 { low , high } . into () }
};
}
