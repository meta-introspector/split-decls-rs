// Generated macro for is_upper_snake_case (function)
macro_rules! Depcrateis_upper_snake_case {
() => {
// Module: crate
// Provides: {"is_upper_snake_case"}
// Dependencies: {}
# [must_use] pub fn is_upper_snake_case (s : & str) -> bool { s . chars () . all (| c | c . is_uppercase () || c == '_' || c . is_numeric ()) }
};
}
