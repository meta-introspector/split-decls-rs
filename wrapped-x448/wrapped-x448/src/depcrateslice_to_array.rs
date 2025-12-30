// Generated macro for slice_to_array (function)
macro_rules! Depcrateslice_to_array {
() => {
// Module: crate
// Provides: {"slice_to_array"}
// Dependencies: {}
fn slice_to_array (bytes : & [u8]) -> [u8 ; 56] { let mut array : [u8 ; 56] = [0 ; 56] ; array . copy_from_slice (bytes) ; array }
};
}
