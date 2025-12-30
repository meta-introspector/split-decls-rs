// Generated macro for test (function)
macro_rules! Depcrate_detect_bittest {
() => {
// Module: crate::detect::bit
// Provides: {"test"}
// Dependencies: {}
# [doc = " Tests the `bit` of `x`."] # [allow (dead_code)] # [inline] pub (crate) fn test (x : usize , bit : u32) -> bool { debug_assert ! (bit < usize :: BITS , "bit index out-of-bounds") ; x & (1 << bit) != 0 }
};
}
