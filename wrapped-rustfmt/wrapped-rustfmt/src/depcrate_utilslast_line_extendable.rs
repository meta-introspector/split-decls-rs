// Generated macro for last_line_extendable (function)
macro_rules! Depcrate_utilslast_line_extendable {
() => {
// Module: crate::utils
// Provides: {"last_line_extendable"}
// Dependencies: {}
# [inline] pub (crate) fn last_line_extendable (s : & str) -> bool { if s . ends_with ("\"#") { return true ; } for c in s . chars () . rev () { match c { '(' | ')' | ']' | '}' | '?' | '>' => continue , '\n' => break , _ if c . is_whitespace () => continue , _ => return false , } } true }
};
}
