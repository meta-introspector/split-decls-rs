macro_rules! deps {
    () => {
        Expected!();
        ErrorSink!();
        ParseError!();
        Raw!();
        Span!();
    };
}

macro_rules! ensure_dec_uint {
    () => {
        deps!();
        pub (crate) fn ensure_dec_uint < 'i > (value : & mut & 'i str , raw : Raw < 'i > , zero_prefix : bool , invalid_description : & 'static str , error : & mut dyn ErrorSink ,) { let start = * value ; let mut digit_count = 0 ; while let Some (current) = value . chars () . next () { if current . is_ascii_digit () { digit_count += 1 ; } else if current == '_' { } else { break ; } let _ = value . next_token () ; } match digit_count { 0 => { let start = start . offset_from (& raw . as_str ()) ; let end = start ; error . report_error (ParseError :: new (invalid_description) . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Description ("digits")]) . with_unexpected (Span :: new_unchecked (start , end)) ,) ; } 1 => { } _ if start . starts_with ("0") && ! zero_prefix => { let start = start . offset_from (& raw . as_str ()) ; let end = start + 1 ; error . report_error (ParseError :: new ("unexpected leading zero") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& []) . with_unexpected (Span :: new_unchecked (start , end)) ,) ; } _ => { } } }
    };
}

ensure_dec_uint!()