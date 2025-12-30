// Generated macro for parse_lit_str_raw (function)
macro_rules! Depcrate_helpers_stringparse_lit_str_raw {
() => {
// Module: crate::helpers::string
// Provides: {"parse_lit_str_raw"}
// Dependencies: {}
fn parse_lit_str_raw (s : & [u8]) -> Vec < u8 > { let mut pounds = 0 ; while byte (s , pounds) == b'#' { pounds += 1 ; } let close = s . iter () . rposition (| & b | b == b'"') . expect ("had a string without trailing \"") ; s [pounds + 1 .. close] . to_owned () }
};
}
