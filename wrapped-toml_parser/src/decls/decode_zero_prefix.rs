macro_rules! deps {
    () => {
        ParseError!();
        StringBuilder!();
        Raw!();
        ScalarKind!();
        Span!();
        Expected!();
        ErrorSink!();
        IntegerRadix!();
    };
}

macro_rules! decode_zero_prefix {
    () => {
        deps!();
        pub (crate) fn decode_zero_prefix < 'i > (value : & 'i str , signed : bool , raw : Raw < 'i > , output : & mut dyn StringBuilder < 'i > , error : & mut dyn ErrorSink ,) -> ScalarKind { debug_assert_eq ! (value . as_bytes () [0] , b'0') ; if value . len () == 1 { let kind = ScalarKind :: Integer (IntegerRadix :: Dec) ; decode_float_or_integer (raw . as_str () , raw , kind , output , error) } else { let radix = value . as_bytes () [1] ; match radix { b'x' | b'X' => { if signed { error . report_error (ParseError :: new ("integers with a radix cannot be signed") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& []) . with_unexpected (Span :: new_unchecked (0 , 1)) ,) ; } if radix == b'X' { let start = value . offset_from (& raw . as_str ()) ; let end = start + 2 ; error . report_error (ParseError :: new ("radix must be lowercase") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Literal ("0x")]) . with_unexpected (Span :: new_unchecked (start , end)) ,) ; } let radix = IntegerRadix :: Hex ; let kind = ScalarKind :: Integer (radix) ; let stream = & value [2 ..] ; ensure_radixed_value (stream , raw , radix , error) ; decode_float_or_integer (stream , raw , kind , output , error) } b'o' | b'O' => { if signed { error . report_error (ParseError :: new ("integers with a radix cannot be signed") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& []) . with_unexpected (Span :: new_unchecked (0 , 1)) ,) ; } if radix == b'O' { let start = value . offset_from (& raw . as_str ()) ; let end = start + 2 ; error . report_error (ParseError :: new ("radix must be lowercase") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Literal ("0o")]) . with_unexpected (Span :: new_unchecked (start , end)) ,) ; } let radix = IntegerRadix :: Oct ; let kind = ScalarKind :: Integer (radix) ; let stream = & value [2 ..] ; ensure_radixed_value (stream , raw , radix , error) ; decode_float_or_integer (stream , raw , kind , output , error) } b'b' | b'B' => { if signed { error . report_error (ParseError :: new ("integers with a radix cannot be signed") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& []) . with_unexpected (Span :: new_unchecked (0 , 1)) ,) ; } if radix == b'B' { let start = value . offset_from (& raw . as_str ()) ; let end = start + 2 ; error . report_error (ParseError :: new ("radix must be lowercase") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Literal ("0b")]) . with_unexpected (Span :: new_unchecked (start , end)) ,) ; } let radix = IntegerRadix :: Bin ; let kind = ScalarKind :: Integer (radix) ; let stream = & value [2 ..] ; ensure_radixed_value (stream , raw , radix , error) ; decode_float_or_integer (stream , raw , kind , output , error) } b'd' | b'D' => { if signed { error . report_error (ParseError :: new ("integers with a radix cannot be signed") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& []) . with_unexpected (Span :: new_unchecked (0 , 1)) ,) ; } let radix = IntegerRadix :: Dec ; let kind = ScalarKind :: Integer (radix) ; let stream = & value [2 ..] ; error . report_error (ParseError :: new ("redundant integer number prefix") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& []) . with_unexpected (Span :: new_unchecked (0 , 2)) ,) ; ensure_radixed_value (stream , raw , radix , error) ; decode_float_or_integer (stream , raw , kind , output , error) } _ => decode_datetime_or_float_or_integer (value , raw , output , error) , } } }
    };
}

decode_zero_prefix!();