// Generated macro for next_pow2 (function)
macro_rules! Depcrate_cfgnext_pow2 {
() => {
// Module: crate::cfg
// Provides: {"next_pow2"}
// Dependencies: {}
pub (crate) const fn next_pow2 (n : usize) -> usize { let pow2 = n . count_ones () == 1 ; let zeros = n . leading_zeros () ; 1 << (WIDTH - zeros as usize - pow2 as usize) }
};
}
