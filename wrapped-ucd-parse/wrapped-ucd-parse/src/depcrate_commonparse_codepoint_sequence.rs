// Generated macro for parse_codepoint_sequence (function)
macro_rules! Depcrate_commonparse_codepoint_sequence {
() => {
// Module: crate::common
// Provides: {"parse_codepoint_sequence"}
// Dependencies: {}
# [doc = " A helper function for parsing a sequence of space separated codepoints."] # [doc = " The sequence is permitted to be empty."] pub fn parse_codepoint_sequence (s : & str) -> Result < Vec < Codepoint > , Error > { let mut cps = vec ! [] ; for cp in s . trim () . split_whitespace () { cps . push (cp . parse () ?) ; } Ok (cps) }
};
}
