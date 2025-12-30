// Generated macro for DOUBLE_WIDTH_CUTOFF (const)
macro_rules! Depcrate_coreDOUBLE_WIDTH_CUTOFF {
() => {
// Module: crate::core
// Provides: {"DOUBLE_WIDTH_CUTOFF"}
// Dependencies: {}
# [doc = " First character which [`ch_width`] will classify as double-width."] # [doc = " Please see [`display_width`]."] # [cfg (not (feature = "unicode-width"))] const DOUBLE_WIDTH_CUTOFF : char = '\u{1100}' ;
};
}
