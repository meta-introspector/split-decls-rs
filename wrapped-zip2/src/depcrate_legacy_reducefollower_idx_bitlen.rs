// Generated macro for follower_idx_bitlen (function)
macro_rules! Depcrate_legacy_reducefollower_idx_bitlen {
() => {
// Module: crate::legacy::reduce
// Provides: {"follower_idx_bitlen"}
// Dependencies: {}
# [doc = " Number of bits used to represent indices in a follower set of size n."] const fn follower_idx_bitlen (n : u8) -> u8 { debug_assert ! (n <= 32) ; match n { 0 => 0 , 1 => 1 , _ => 8 - (n - 1) . leading_zeros () as u8 , } }
};
}
