// Generated macro for numeric_extended_from (function)
macro_rules! Depcrate_headernumeric_extended_from {
() => {
// Module: crate::header
// Provides: {"numeric_extended_from"}
// Dependencies: {}
fn numeric_extended_from (src : & [u8]) -> u64 { let mut dst : u64 = 0 ; let mut b_to_skip = 1 ; if src . len () == 8 { dst = (src [0] ^ 0x80) as u64 ; } else { b_to_skip = src . len () - 8 ; } for byte in src . iter () . skip (b_to_skip) { dst <<= 8 ; dst |= * byte as u64 ; } dst }
};
}
