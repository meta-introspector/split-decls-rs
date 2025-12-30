// Generated macro for filtered_str_fits (function)
macro_rules! Depcrate_utilsfiltered_str_fits {
() => {
// Module: crate::utils
// Provides: {"filtered_str_fits"}
// Dependencies: {}
pub (crate) fn filtered_str_fits (snippet : & str , max_width : usize , shape : Shape) -> bool { let snippet = & filter_normal_code (snippet) ; if ! snippet . is_empty () { if first_line_width (snippet) > shape . width { return false ; } if is_single_line (snippet) { return true ; } if snippet . lines () . skip (1) . any (| line | unicode_str_width (line) > max_width) { return false ; } if last_line_width (snippet) > shape . used_width () + shape . width { return false ; } } true }
};
}
