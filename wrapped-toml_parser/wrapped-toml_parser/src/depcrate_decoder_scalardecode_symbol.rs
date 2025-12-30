// Generated macro for decode_symbol (function)
macro_rules! Depcrate_decoder_scalardecode_symbol {
() => {
// Module: crate::decoder::scalar
// Provides: {"decode_symbol"}
// Dependencies: {}
pub (crate) fn decode_symbol < 'i > (raw : Raw < 'i > , symbol : & 'static str , kind : ScalarKind , expected : & 'static [Expected] , output : & mut dyn StringBuilder < 'i > , error : & mut dyn ErrorSink ,) -> ScalarKind { if raw . as_str () != symbol { if raw . as_str () . contains (" ") { return decode_invalid (raw , output , error) ; } else { error . report_error (ParseError :: new (kind . invalid_description ()) . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (expected) . with_unexpected (Span :: new_unchecked (0 , raw . len ())) ,) ; } } decode_as (raw , symbol , kind , output , error) }
};
}
