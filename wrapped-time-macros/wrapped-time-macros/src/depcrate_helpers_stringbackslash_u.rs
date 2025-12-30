// Generated macro for backslash_u (function)
macro_rules! Depcrate_helpers_stringbackslash_u {
() => {
// Module: crate::helpers::string
// Provides: {"backslash_u"}
// Dependencies: {}
fn backslash_u (mut s : & str) -> (char , & str) { s = & s [1 ..] ; let mut ch = 0 ; let mut digits = 0 ; loop { let b = byte (s , 0) ; let digit = match b { b'0' ..= b'9' => b - b'0' , b'a' ..= b'f' => 10 + b - b'a' , b'A' ..= b'F' => 10 + b - b'A' , b'_' if digits > 0 => { s = & s [1 ..] ; continue ; } b'}' if digits != 0 => break , _ => bug ! ("invalid unicode escape") , } ; ch *= 0x10 ; ch += u32 :: from (digit) ; digits += 1 ; s = & s [1 ..] ; } s = & s [1 ..] ; (char :: from_u32 (ch) . expect ("invalid unicode escape passed by compiler") , s ,) }
};
}
