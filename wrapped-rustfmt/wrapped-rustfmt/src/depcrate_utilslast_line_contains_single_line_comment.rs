// Generated macro for last_line_contains_single_line_comment (function)
macro_rules! Depcrate_utilslast_line_contains_single_line_comment {
() => {
// Module: crate::utils
// Provides: {"last_line_contains_single_line_comment"}
// Dependencies: {}
# [inline] pub (crate) fn last_line_contains_single_line_comment (s : & str) -> bool { s . lines () . last () . map_or (false , | l | l . contains ("//")) }
};
}
