// Generated macro for unset_bit (function)
macro_rules! Depcrate_detect_cacheunset_bit {
() => {
// Module: crate::detect::cache
// Provides: {"unset_bit"}
// Dependencies: {}
# [doc = " Unset the `bit of `x`."] # [inline] const fn unset_bit (x : u128 , bit : u32) -> u128 { x & ! (1 << bit) }
};
}
