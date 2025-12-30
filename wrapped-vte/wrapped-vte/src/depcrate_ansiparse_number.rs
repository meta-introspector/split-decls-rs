// Generated macro for parse_number (function)
macro_rules! Depcrate_ansiparse_number {
() => {
// Module: crate::ansi
// Provides: {"parse_number"}
// Dependencies: {}
fn parse_number (input : & [u8]) -> Option < u8 > { if input . is_empty () { return None ; } let mut num : u8 = 0 ; for c in input { let c = * c as char ; let digit = c . to_digit (10) ? ; num = num . checked_mul (10) . and_then (| v | v . checked_add (digit as u8)) ? ; } Some (num) }
};
}
