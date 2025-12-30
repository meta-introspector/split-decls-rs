// Generated macro for is_snake_case (function)
macro_rules! Depcrate_helpersis_snake_case {
() => {
// Module: crate::helpers
// Provides: {"is_snake_case"}
// Dependencies: {}
pub fn is_snake_case (s : & str) -> bool { s . chars () . all (| c | c . is_ascii_lowercase () || c . is_ascii_digit () || c == '_') }
};
}
