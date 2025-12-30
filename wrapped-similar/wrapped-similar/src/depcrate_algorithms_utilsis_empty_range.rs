// Generated macro for is_empty_range (function)
macro_rules! Depcrate_algorithms_utilsis_empty_range {
() => {
// Module: crate::algorithms::utils
// Provides: {"is_empty_range"}
// Dependencies: {}
# [doc = " Utility function to check if a range is empty that works on older rust versions"] # [inline (always)] # [allow (clippy :: neg_cmp_op_on_partial_ord)] pub fn is_empty_range < T : PartialOrd < T > > (range : & Range < T >) -> bool { ! (range . start < range . end) }
};
}
