macro_rules! deps {
    () => {
        Expected!();
        ErrorSink!();
        Raw!();
        ScalarKind!();
        StringBuilder!();
    };
}

macro_rules! decode_unquoted_scalar {
    () => {
        deps!();
        pub (crate) fn decode_unquoted_scalar < 'i > (raw : Raw < 'i > , output : & mut dyn StringBuilder < 'i > , error : & mut dyn ErrorSink ,) -> ScalarKind { let s = raw . as_str () ; let Some (first) = s . as_bytes () . first () else { return decode_invalid (raw , output , error) ; } ; match first { b'+' | b'-' => { let value = & raw . as_str () [1 ..] ; decode_sign_prefix (raw , value , output , error) } b'_' => decode_datetime_or_float_or_integer (raw . as_str () , raw , output , error) , b'0' => decode_zero_prefix (raw . as_str () , false , raw , output , error) , b'1' ..= b'9' => decode_datetime_or_float_or_integer (raw . as_str () , raw , output , error) , b'.' => { let kind = ScalarKind :: Float ; let stream = raw . as_str () ; ensure_float (stream , raw , error) ; decode_float_or_integer (stream , raw , kind , output , error) } b't' | b'T' => { const SYMBOL : & str = "true" ; let kind = ScalarKind :: Boolean (true) ; let expected = & [Expected :: Literal (SYMBOL)] ; decode_symbol (raw , SYMBOL , kind , expected , output , error) } b'f' | b'F' => { const SYMBOL : & str = "false" ; let kind = ScalarKind :: Boolean (false) ; let expected = & [Expected :: Literal (SYMBOL)] ; decode_symbol (raw , SYMBOL , kind , expected , output , error) } b'i' | b'I' => { const SYMBOL : & str = "inf" ; let kind = ScalarKind :: Float ; let expected = & [Expected :: Literal (SYMBOL)] ; decode_symbol (raw , SYMBOL , kind , expected , output , error) } b'n' | b'N' => { const SYMBOL : & str = "nan" ; let kind = ScalarKind :: Float ; let expected = & [Expected :: Literal (SYMBOL)] ; decode_symbol (raw , SYMBOL , kind , expected , output , error) } _ => decode_invalid (raw , output , error) , } }
    };
}

decode_unquoted_scalar!();