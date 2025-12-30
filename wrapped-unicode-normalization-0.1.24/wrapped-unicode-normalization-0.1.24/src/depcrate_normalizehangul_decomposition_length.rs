// Generated macro for hangul_decomposition_length (function)
macro_rules! Depcrate_normalizehangul_decomposition_length {
() => {
// Module: crate::normalize
// Provides: {"hangul_decomposition_length"}
// Dependencies: {}
# [inline] pub (crate) fn hangul_decomposition_length (s : char) -> usize { let si = s as u32 - S_BASE ; let ti = si % T_COUNT ; if ti > 0 { 3 } else { 2 } }
};
}
