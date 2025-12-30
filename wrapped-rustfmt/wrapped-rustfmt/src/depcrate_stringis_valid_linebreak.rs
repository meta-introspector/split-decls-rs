// Generated macro for is_valid_linebreak (function)
macro_rules! Depcrate_stringis_valid_linebreak {
() => {
// Module: crate::string
// Provides: {"is_valid_linebreak"}
// Dependencies: {}
fn is_valid_linebreak (input : & [& str] , pos : usize) -> bool { let is_whitespace = is_whitespace (input [pos]) ; if is_whitespace { return true ; } let is_punctuation = is_punctuation (input [pos]) ; if is_punctuation && ! is_part_of_type (input , pos) { return true ; } false }
};
}
