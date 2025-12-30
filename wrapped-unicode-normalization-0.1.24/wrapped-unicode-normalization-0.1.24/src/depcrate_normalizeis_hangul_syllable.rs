// Generated macro for is_hangul_syllable (function)
macro_rules! Depcrate_normalizeis_hangul_syllable {
() => {
// Module: crate::normalize
// Provides: {"is_hangul_syllable"}
// Dependencies: {}
pub (crate) fn is_hangul_syllable (c : char) -> bool { (c as u32) >= S_BASE && (c as u32) < (S_BASE + S_COUNT) }
};
}
