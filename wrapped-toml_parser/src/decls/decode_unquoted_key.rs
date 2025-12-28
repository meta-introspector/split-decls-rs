macro_rules! deps {
    () => {
        ErrorSink!();
        Span!();
        Expected!();
        StringBuilder!();
        Raw!();
        ParseError!();
    };
}

macro_rules! decode_unquoted_key {
    () => {
        deps!();
        # [doc = " Parse unquoted key"] # [doc = ""] # [doc = " ```bnf"] # [doc = " unquoted-key = 1*( ALPHA / DIGIT / %x2D / %x5F ) ; A-Z / a-z / 0-9 / - / _"] # [doc = " ```"] pub (crate) fn decode_unquoted_key < 'i > (raw : Raw < 'i > , output : & mut dyn StringBuilder < 'i > , error : & mut dyn ErrorSink ,) { let s = raw . as_str () ; if s . is_empty () { error . report_error (ParseError :: new ("unquoted keys cannot be empty") . with_context (Span :: new_unchecked (0 , s . len ())) . with_expected (& [Expected :: Description ("letters") , Expected :: Description ("numbers") , Expected :: Literal ("-") , Expected :: Literal ("_") ,]) . with_unexpected (Span :: new_unchecked (0 , s . len ())) ,) ; } for (i , b) in s . as_bytes () . iter () . enumerate () { if ! UNQUOTED_CHAR . contains_token (b) { error . report_error (ParseError :: new ("invalid unquoted key") . with_context (Span :: new_unchecked (0 , s . len ())) . with_expected (& [Expected :: Description ("letters") , Expected :: Description ("numbers") , Expected :: Literal ("-") , Expected :: Literal ("_") ,]) . with_unexpected (Span :: new_unchecked (i , i)) ,) ; } } if ! output . push_str (s) { error . report_error (ParseError :: new (ALLOCATION_ERROR) . with_unexpected (Span :: new_unchecked (0 , raw . len ())) ,) ; } }
    };
}

decode_unquoted_key!()