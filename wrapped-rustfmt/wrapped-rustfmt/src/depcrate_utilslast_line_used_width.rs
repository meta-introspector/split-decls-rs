// Generated macro for last_line_used_width (function)
macro_rules! Depcrate_utilslast_line_used_width {
() => {
// Module: crate::utils
// Provides: {"last_line_used_width"}
// Dependencies: {}
# [doc = " The total used width of the last line."] # [inline] pub (crate) fn last_line_used_width (s : & str , offset : usize) -> usize { if s . contains ('\n') { last_line_width (s) } else { offset + unicode_str_width (s) } }
};
}
