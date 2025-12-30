// Generated macro for last_line_width (function)
macro_rules! Depcrate_utilslast_line_width {
() => {
// Module: crate::utils
// Provides: {"last_line_width"}
// Dependencies: {}
# [doc = " The width of the last line in s."] # [inline] pub (crate) fn last_line_width (s : & str) -> usize { unicode_str_width (s . rsplitn (2 , '\n') . next () . unwrap_or ("")) }
};
}
