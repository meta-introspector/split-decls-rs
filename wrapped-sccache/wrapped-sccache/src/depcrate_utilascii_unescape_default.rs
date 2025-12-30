// Generated macro for ascii_unescape_default (function)
macro_rules! Depcrate_utilascii_unescape_default {
() => {
// Module: crate::util
// Provides: {"ascii_unescape_default"}
// Dependencies: {}
# [doc = " A reverse version of std::ascii::escape_default"] pub fn ascii_unescape_default (s : & [u8]) -> std :: io :: Result < Vec < u8 > > { let mut out = Vec :: with_capacity (s . len () + 4) ; let mut offset = 0 ; while offset < s . len () { let c = s [offset] ; if c == b'\\' { offset += 1 ; if offset >= s . len () { return Err (std :: io :: Error :: new (std :: io :: ErrorKind :: InvalidInput , "incomplete escape" ,)) ; } let c = s [offset] ; match c { b'n' => out . push (b'\n') , b'r' => out . push (b'\r') , b't' => out . push (b'\t') , b'\'' => out . push (b'\'') , b'"' => out . push (b'"') , b'\\' => out . push (b'\\') , b'x' => { offset += 1 ; if offset + 1 >= s . len () { return Err (std :: io :: Error :: new (std :: io :: ErrorKind :: InvalidInput , "incomplete hex escape" ,)) ; } let v = (unhex (s [offset]) ? << 4) | unhex (s [offset + 1]) ? ; out . push (v) ; offset += 1 ; } _ => { return Err (std :: io :: Error :: new (std :: io :: ErrorKind :: InvalidInput , "invalid escape" ,)) ; } } } else { out . push (c) ; } offset += 1 ; } Ok (out) }
};
}
