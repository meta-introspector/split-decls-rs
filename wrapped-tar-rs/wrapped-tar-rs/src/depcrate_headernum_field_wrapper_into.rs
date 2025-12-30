// Generated macro for num_field_wrapper_into (function)
macro_rules! Depcrate_headernum_field_wrapper_into {
() => {
// Module: crate::header
// Provides: {"num_field_wrapper_into"}
// Dependencies: {}
fn num_field_wrapper_into (dst : & mut [u8] , src : u64) { if src >= 8589934592 || (src >= 2097152 && dst . len () == 8) { numeric_extended_into (dst , src) ; } else { octal_into (dst , src) ; } }
};
}
