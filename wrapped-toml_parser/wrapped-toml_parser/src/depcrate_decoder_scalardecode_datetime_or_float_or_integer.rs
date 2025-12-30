// Generated macro for decode_datetime_or_float_or_integer (function)
macro_rules! Depcrate_decoder_scalardecode_datetime_or_float_or_integer {
() => {
// Module: crate::decoder::scalar
// Provides: {"decode_datetime_or_float_or_integer"}
// Dependencies: {}
pub (crate) fn decode_datetime_or_float_or_integer < 'i > (value : & 'i str , raw : Raw < 'i > , output : & mut dyn StringBuilder < 'i > , error : & mut dyn ErrorSink ,) -> ScalarKind { let Some (digit_end) = value . as_bytes () . offset_for (| b | ! (b'0' ..= b'9') . contains_token (b)) else { let kind = ScalarKind :: Integer (IntegerRadix :: Dec) ; let stream = raw . as_str () ; ensure_no_leading_zero (value , raw , error) ; return decode_float_or_integer (stream , raw , kind , output , error) ; } ; # [cfg (feature = "unsafe")] let rest = unsafe { & value . get_unchecked (digit_end ..) } ; # [cfg (not (feature = "unsafe"))] let rest = & value [digit_end ..] ; if rest . starts_with ("-") || rest . starts_with (":") { decode_as_is (raw , ScalarKind :: DateTime , output , error) } else if rest . contains (" ") { decode_invalid (raw , output , error) } else if is_float (rest) { let kind = ScalarKind :: Float ; let stream = raw . as_str () ; ensure_float (value , raw , error) ; decode_float_or_integer (stream , raw , kind , output , error) } else if rest . starts_with ("_") { let kind = ScalarKind :: Integer (IntegerRadix :: Dec) ; let stream = raw . as_str () ; ensure_no_leading_zero (value , raw , error) ; decode_float_or_integer (stream , raw , kind , output , error) } else { decode_invalid (raw , output , error) } }
};
}
