// Generated macro for decode_as_is (function)
macro_rules! Depcrate_decoder_scalardecode_as_is {
() => {
// Module: crate::decoder::scalar
// Provides: {"decode_as_is"}
// Dependencies: {}
pub (crate) fn decode_as_is < 'i > (raw : Raw < 'i > , kind : ScalarKind , output : & mut dyn StringBuilder < 'i > , error : & mut dyn ErrorSink ,) -> ScalarKind { let kind = decode_as (raw , raw . as_str () , kind , output , error) ; kind }
};
}
