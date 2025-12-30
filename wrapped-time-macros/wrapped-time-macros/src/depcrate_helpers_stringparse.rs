// Generated macro for parse (function)
macro_rules! Depcrate_helpers_stringparse {
() => {
// Module: crate::helpers::string
// Provides: {"parse"}
// Dependencies: {}
pub (crate) fn parse (token : & proc_macro :: Literal) -> Result < (Span , Vec < u8 >) , Error > { let span = token . span () ; let repr = token . to_string () ; match repr . as_bytes () { [b'"' , ..] => Ok ((span , parse_lit_str_cooked (& repr [1 ..]))) , [b'b' , b'"' , rest @ ..] => Ok ((span , parse_lit_byte_str_cooked (rest))) , [b'r' , rest @ ..] | [b'b' , b'r' , rest @ ..] => Ok ((span , parse_lit_str_raw (rest))) , _ => Err (Error :: ExpectedString { span_start : Some (span) , span_end : Some (span) , }) , } }
};
}
