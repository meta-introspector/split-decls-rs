// Generated macro for is_dec_integer_digit (function)
macro_rules! Depcrate_decoder_scalaris_dec_integer_digit {
() => {
// Module: crate::decoder::scalar
// Provides: {"is_dec_integer_digit"}
// Dependencies: {}
fn is_dec_integer_digit (b : u8) -> bool { (b'0' ..= b'9') . contains_token (b) }
};
}
