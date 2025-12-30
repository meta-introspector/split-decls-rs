// Generated macro for first_line_width (function)
macro_rules! Depcrate_utilsfirst_line_width {
() => {
// Module: crate::utils
// Provides: {"first_line_width"}
// Dependencies: {}
# [doc = " The width of the first line in s."] # [inline] pub (crate) fn first_line_width (s : & str) -> usize { unicode_str_width (s . splitn (2 , '\n') . next () . unwrap_or ("")) }
};
}
