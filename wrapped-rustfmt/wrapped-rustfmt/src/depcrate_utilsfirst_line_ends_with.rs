// Generated macro for first_line_ends_with (function)
macro_rules! Depcrate_utilsfirst_line_ends_with {
() => {
// Module: crate::utils
// Provides: {"first_line_ends_with"}
// Dependencies: {}
# [inline] pub (crate) fn first_line_ends_with (s : & str , c : char) -> bool { s . lines () . next () . map_or (false , | l | l . ends_with (c)) }
};
}
