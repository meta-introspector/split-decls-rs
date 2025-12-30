// Generated macro for is_any_integer_digit (function)
macro_rules! Depcrate_decoder_scalaris_any_integer_digit {
() => {
// Module: crate::decoder::scalar
// Provides: {"is_any_integer_digit"}
// Dependencies: {}
fn is_any_integer_digit (b : u8) -> bool { (b'0' ..= b'9' , b'a' ..= b'f' , b'A' ..= b'F') . contains_token (b) }
};
}
