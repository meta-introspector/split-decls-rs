// Generated macro for has_underscore (function)
macro_rules! Depcrate_decoder_scalarhas_underscore {
() => {
// Module: crate::decoder::scalar
// Provides: {"has_underscore"}
// Dependencies: {}
fn has_underscore (raw : & str) -> bool { raw . as_bytes () . find_slice (b'_') . is_some () }
};
}
