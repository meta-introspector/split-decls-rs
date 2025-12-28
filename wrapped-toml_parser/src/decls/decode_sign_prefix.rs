macro_rules! deps {
    () => {
        ErrorSink!();
        ScalarKind!();
        Expected!();
        ParseError!();
        Raw!();
        StringBuilder!();
        Span!();
    };
}

macro_rules! decode_sign_prefix {
    () => {
        deps!();
        pub (crate) fn decode_sign_prefix < 'i > (raw : Raw < 'i > , value : & 'i str , output : & mut dyn StringBuilder < 'i > , error : & mut dyn ErrorSink ,) -> ScalarKind { let Some (first) = value . as_bytes () . first () else { return decode_invalid (raw , output , error) ; } ; match first { b'+' | b'-' => { let start = value . offset_from (& raw . as_str ()) ; let end = start + 1 ; error . report_error (ParseError :: new ("redundant numeric sign") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& []) . with_unexpected (Span :: new_unchecked (start , end)) ,) ; let value = & value [1 ..] ; decode_sign_prefix (raw , value , output , error) } b'_' => decode_datetime_or_float_or_integer (value , raw , output , error) , b'0' => decode_zero_prefix (value , true , raw , output , error) , b'1' ..= b'9' => decode_datetime_or_float_or_integer (value , raw , output , error) , b'.' => { let kind = ScalarKind :: Float ; let stream = raw . as_str () ; ensure_float (stream , raw , error) ; decode_float_or_integer (stream , raw , kind , output , error) } b'i' | b'I' => { const SYMBOL : & str = "inf" ; let kind = ScalarKind :: Float ; if value != SYMBOL { let expected = & [Expected :: Literal (SYMBOL)] ; let start = value . offset_from (& raw . as_str ()) ; let end = start + value . len () ; error . report_error (ParseError :: new (kind . invalid_description ()) . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (expected) . with_unexpected (Span :: new_unchecked (start , end)) ,) ; decode_as (raw , SYMBOL , kind , output , error) } else { decode_as_is (raw , kind , output , error) } } b'n' | b'N' => { const SYMBOL : & str = "nan" ; let kind = ScalarKind :: Float ; if value != SYMBOL { let expected = & [Expected :: Literal (SYMBOL)] ; let start = value . offset_from (& raw . as_str ()) ; let end = start + value . len () ; error . report_error (ParseError :: new (kind . invalid_description ()) . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (expected) . with_unexpected (Span :: new_unchecked (start , end)) ,) ; decode_as (raw , SYMBOL , kind , output , error) } else { decode_as_is (raw , kind , output , error) } } _ => decode_invalid (raw , output , error) , } }
    };
}

decode_sign_prefix!()