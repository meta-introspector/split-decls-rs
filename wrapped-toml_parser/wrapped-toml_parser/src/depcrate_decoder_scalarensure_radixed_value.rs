// Generated macro for ensure_radixed_value (function)
macro_rules! Depcrate_decoder_scalarensure_radixed_value {
() => {
// Module: crate::decoder::scalar
// Provides: {"ensure_radixed_value"}
// Dependencies: {}
pub (crate) fn ensure_radixed_value (value : & str , raw : Raw < '_ > , radix : IntegerRadix , error : & mut dyn ErrorSink ,) { let invalid = ['+' , '-'] ; let value = if let Some (value) = value . strip_prefix (invalid) { let pos = raw . as_str () . find (invalid) . unwrap () ; error . report_error (ParseError :: new ("unexpected sign") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& []) . with_unexpected (Span :: new_unchecked (pos , pos + 1)) ,) ; value } else { value } ; let valid = radix . validator () ; for (index , c) in value . char_indices () { if ! valid (c) && c != '_' { let pos = value . offset_from (& raw . as_str ()) + index ; error . report_error (ParseError :: new (radix . invalid_description ()) . with_context (Span :: new_unchecked (0 , raw . len ())) . with_unexpected (Span :: new_unchecked (pos , pos)) ,) ; } } }
};
}
