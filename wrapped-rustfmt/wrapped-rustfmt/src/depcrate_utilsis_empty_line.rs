// Generated macro for is_empty_line (function)
macro_rules! Depcrate_utilsis_empty_line {
() => {
// Module: crate::utils
// Provides: {"is_empty_line"}
// Dependencies: {}
pub (crate) fn is_empty_line (s : & str) -> bool { s . is_empty () || s . chars () . all (char :: is_whitespace) }
};
}
