// Generated macro for is_whitespace (function)
macro_rules! Depcrate_stringis_whitespace {
() => {
// Module: crate::string
// Provides: {"is_whitespace"}
// Dependencies: {}
fn is_whitespace (grapheme : & str) -> bool { grapheme . chars () . all (char :: is_whitespace) }
};
}
