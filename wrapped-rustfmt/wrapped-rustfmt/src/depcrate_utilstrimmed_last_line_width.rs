// Generated macro for trimmed_last_line_width (function)
macro_rules! Depcrate_utilstrimmed_last_line_width {
() => {
// Module: crate::utils
// Provides: {"trimmed_last_line_width"}
// Dependencies: {}
# [inline] pub (crate) fn trimmed_last_line_width (s : & str) -> usize { unicode_str_width (match s . rfind ('\n') { Some (n) => s [(n + 1) ..] . trim () , None => s . trim () , }) }
};
}
