// Generated macro for parse_lit_str_cooked (function)
macro_rules! Depcrate_helpers_stringparse_lit_str_cooked {
() => {
// Module: crate::helpers::string
// Provides: {"parse_lit_str_cooked"}
// Dependencies: {}
fn parse_lit_str_cooked (mut s : & str) -> Vec < u8 > { let mut content = String :: new () ; 'outer : loop { let ch = match byte (s , 0) { b'"' => break , b'\\' => { let b = byte (s , 1) ; s = & s [2 ..] ; match b { b'x' => { let (byte , rest) = backslash_x (s) ; s = rest ; char :: from_u32 (u32 :: from (byte)) . expect ("byte was just validated") } b'u' => { let (chr , rest) = backslash_u (s) ; s = rest ; chr } b'n' => '\n' , b'r' => '\r' , b't' => '\t' , b'\\' => '\\' , b'0' => '\0' , b'\'' => '\'' , b'"' => '"' , b'\r' | b'\n' => loop { let ch = s . chars () . next () . unwrap_or_default () ; if ch . is_whitespace () { s = & s [ch . len_utf8 () ..] ; } else { continue 'outer ; } } , _ => bug ! ("invalid escape") , } } b'\r' => { s = & s [2 ..] ; '\n' } _ => { let ch = s . chars () . next () . unwrap_or_default () ; s = & s [ch . len_utf8 () ..] ; ch } } ; content . push (ch) ; } content . into_bytes () }
};
}
