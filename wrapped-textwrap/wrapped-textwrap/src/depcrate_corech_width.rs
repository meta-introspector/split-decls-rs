// Generated macro for ch_width (function)
macro_rules! Depcrate_corech_width {
() => {
// Module: crate::core
// Provides: {"ch_width"}
// Dependencies: {}
# [cfg (not (feature = "unicode-width"))] # [inline] fn ch_width (ch : char) -> usize { if ch < DOUBLE_WIDTH_CUTOFF { 1 } else { 2 } }
};
}
