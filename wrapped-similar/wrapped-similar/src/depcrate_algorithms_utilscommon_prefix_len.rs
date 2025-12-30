// Generated macro for common_prefix_len (function)
macro_rules! Depcrate_algorithms_utilscommon_prefix_len {
() => {
// Module: crate::algorithms::utils
// Provides: {"common_prefix_len"}
// Dependencies: {}
# [doc = " Given two lookups and ranges calculates the length of the common prefix."] pub fn common_prefix_len < Old , New > (old : & Old , old_range : Range < usize > , new : & New , new_range : Range < usize > ,) -> usize where Old : Index < usize > + ? Sized , New : Index < usize > + ? Sized , New :: Output : PartialEq < Old :: Output > , { if is_empty_range (& old_range) || is_empty_range (& new_range) { return 0 ; } new_range . zip (old_range) . take_while (# [inline (always)] | x | new [x . 0] == old [x . 1] ,) . count () }
};
}
