// Generated macro for parse_lit_byte_str_cooked (function)
macro_rules! Depcrate_helpers_stringparse_lit_byte_str_cooked {
() => {
// Module: crate::helpers::string
// Provides: {"parse_lit_byte_str_cooked"}
// Dependencies: {}
fn parse_lit_byte_str_cooked (mut v : & [u8]) -> Vec < u8 > { let mut out = Vec :: new () ; 'outer : loop { let byte = match byte (v , 0) { b'"' => break , b'\\' => { let b = byte (v , 1) ; v = & v [2 ..] ; match b { b'x' => { let (byte , rest) = backslash_x (v) ; v = rest ; byte } b'n' => b'\n' , b'r' => b'\r' , b't' => b'\t' , b'\\' => b'\\' , b'0' => b'\0' , b'\'' => b'\'' , b'"' => b'"' , b'\r' | b'\n' => loop { let byte = byte (v , 0) ; let ch = char :: from_u32 (u32 :: from (byte)) . expect ("invalid byte") ; if ch . is_whitespace () { v = & v [1 ..] ; } else { continue 'outer ; } } , _ => bug ! ("invalid escape") , } } b'\r' => { v = & v [2 ..] ; b'\n' } b => { v = & v [1 ..] ; b } } ; out . push (byte) ; } out }
};
}
