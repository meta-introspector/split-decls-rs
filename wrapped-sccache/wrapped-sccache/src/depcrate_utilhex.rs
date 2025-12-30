// Generated macro for hex (function)
macro_rules! Depcrate_utilhex {
() => {
// Module: crate::util
// Provides: {"hex"}
// Dependencies: {}
pub fn hex (bytes : & [u8]) -> String { let mut s = String :: with_capacity (bytes . len () * 2) ; for & byte in bytes { s . push (hex (byte & 0xf)) ; s . push (hex ((byte >> 4) & 0xf)) ; } return s ; fn hex (byte : u8) -> char { match byte { 0 ..= 9 => (b'0' + byte) as char , _ => (b'a' + byte - 10) as char , } } }
};
}
