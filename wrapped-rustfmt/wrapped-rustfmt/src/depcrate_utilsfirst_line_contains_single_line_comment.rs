// Generated macro for first_line_contains_single_line_comment (function)
macro_rules! Depcrate_utilsfirst_line_contains_single_line_comment {
() => {
// Module: crate::utils
// Provides: {"first_line_contains_single_line_comment"}
// Dependencies: {}
# [inline] pub (crate) fn first_line_contains_single_line_comment (s : & str) -> bool { s . lines () . next () . map_or (false , | l | l . contains ("//")) }
};
}
