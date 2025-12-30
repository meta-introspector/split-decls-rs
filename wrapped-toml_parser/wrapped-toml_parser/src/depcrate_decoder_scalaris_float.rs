// Generated macro for is_float (function)
macro_rules! Depcrate_decoder_scalaris_float {
() => {
// Module: crate::decoder::scalar
// Provides: {"is_float"}
// Dependencies: {}
fn is_float (raw : & str) -> bool { raw . as_bytes () . find_slice ((b'.' , b'e' , b'E')) . is_some () }
};
}
