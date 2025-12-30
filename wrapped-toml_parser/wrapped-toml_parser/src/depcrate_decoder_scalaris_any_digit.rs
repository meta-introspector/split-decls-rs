// Generated macro for is_any_digit (function)
macro_rules! Depcrate_decoder_scalaris_any_digit {
() => {
// Module: crate::decoder::scalar
// Provides: {"is_any_digit"}
// Dependencies: {}
fn is_any_digit (b : u8 , kind : ScalarKind) -> bool { if kind == ScalarKind :: Float { is_dec_integer_digit (b) } else { is_any_integer_digit (b) } }
};
}
