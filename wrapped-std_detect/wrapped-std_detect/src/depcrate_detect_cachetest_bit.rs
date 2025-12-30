// Generated macro for test_bit (function)
macro_rules! Depcrate_detect_cachetest_bit {
() => {
// Module: crate::detect::cache
// Provides: {"test_bit"}
// Dependencies: {}
# [doc = " Tests the `bit` of `x`."] # [inline] const fn test_bit (x : u128 , bit : u32) -> bool { x & (1 << bit) != 0 }
};
}
