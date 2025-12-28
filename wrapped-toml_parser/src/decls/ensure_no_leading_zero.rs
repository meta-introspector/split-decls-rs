macro_rules! deps {
    () => {
        ErrorSink!();
        Raw!();
        Span!();
        ParseError!();
    };
}

macro_rules! ensure_no_leading_zero {
    () => {
        deps!();
        pub (crate) fn ensure_no_leading_zero < 'i > (value : & 'i str , raw : Raw < 'i > , error : & mut dyn ErrorSink) { if value . starts_with ("0") { let start = value . offset_from (& raw . as_str ()) ; let end = start + 1 ; error . report_error (ParseError :: new ("unexpected leading zero") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& []) . with_unexpected (Span :: new_unchecked (start , end)) ,) ; } }
    };
}

ensure_no_leading_zero!();