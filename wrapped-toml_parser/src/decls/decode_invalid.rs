macro_rules! deps {
    () => {
        StringBuilder!();
        ErrorSink!();
        ScalarKind!();
        Expected!();
        Span!();
        ParseError!();
        Raw!();
    };
}

macro_rules! decode_invalid {
    () => {
        deps!();
        pub (crate) fn decode_invalid < 'i > (raw : Raw < 'i > , output : & mut dyn StringBuilder < 'i > , error : & mut dyn ErrorSink ,) -> ScalarKind { if raw . as_str () . ends_with ("'''") { error . report_error (ParseError :: new ("missing opening quote") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Literal (r#"'''"#)]) . with_unexpected (Span :: new_unchecked (0 , 0)) ,) ; } else if raw . as_str () . ends_with (r#"""""#) { error . report_error (ParseError :: new ("missing opening quote") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Description ("multi-line basic string")]) . with_expected (& [Expected :: Literal (r#"""""#)]) . with_unexpected (Span :: new_unchecked (0 , 0)) ,) ; } else if raw . as_str () . ends_with ("'") { error . report_error (ParseError :: new ("missing opening quote") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Literal (r#"'"#)]) . with_unexpected (Span :: new_unchecked (0 , 0)) ,) ; } else if raw . as_str () . ends_with (r#"""#) { error . report_error (ParseError :: new ("missing opening quote") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Literal (r#"""#)]) . with_unexpected (Span :: new_unchecked (0 , 0)) ,) ; } else { error . report_error (ParseError :: new ("string values must be quoted") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Description ("literal string")]) . with_unexpected (Span :: new_unchecked (0 , raw . len ())) ,) ; } output . clear () ; if ! output . push_str (raw . as_str ()) { error . report_error (ParseError :: new (ALLOCATION_ERROR) . with_unexpected (Span :: new_unchecked (0 , raw . len ())) ,) ; } ScalarKind :: String }
    };
}

decode_invalid!()