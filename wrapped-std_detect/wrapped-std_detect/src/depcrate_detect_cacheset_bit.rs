// Generated macro for set_bit (function)
macro_rules! Depcrate_detect_cacheset_bit {
() => {
// Module: crate::detect::cache
// Provides: {"set_bit"}
// Dependencies: {}
# [doc = " Sets the `bit` of `x`."] # [inline] const fn set_bit (x : u128 , bit : u32) -> u128 { x | 1 << bit }
};
}
