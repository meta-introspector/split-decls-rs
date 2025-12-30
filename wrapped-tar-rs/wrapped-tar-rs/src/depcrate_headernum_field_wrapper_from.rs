// Generated macro for num_field_wrapper_from (function)
macro_rules! Depcrate_headernum_field_wrapper_from {
() => {
// Module: crate::header
// Provides: {"num_field_wrapper_from"}
// Dependencies: {}
fn num_field_wrapper_from (src : & [u8]) -> io :: Result < u64 > { if src [0] & 0x80 != 0 { Ok (numeric_extended_from (src)) } else { octal_from (src) } }
};
}
