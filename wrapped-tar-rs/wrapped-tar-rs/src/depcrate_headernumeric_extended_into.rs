// Generated macro for numeric_extended_into (function)
macro_rules! Depcrate_headernumeric_extended_into {
() => {
// Module: crate::header
// Provides: {"numeric_extended_into"}
// Dependencies: {}
fn numeric_extended_into (dst : & mut [u8] , src : u64) { let len : usize = dst . len () ; for (slot , val) in dst . iter_mut () . zip (repeat (0) . take (len - 8) . chain ((0 .. 8) . rev () . map (| x | ((src >> (8 * x)) & 0xff) as u8)) ,) { * slot = val ; } dst [0] |= 0x80 ; }
};
}
