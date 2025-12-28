macro_rules! deps {
    () => {
        Span!();
        Raw!();
        ScalarKind!();
        StringBuilder!();
        ParseError!();
        ErrorSink!();
    };
}

macro_rules! decode_float_or_integer {
    () => {
        deps!();
        pub (crate) fn decode_float_or_integer < 'i > (stream : & 'i str , raw : Raw < 'i > , kind : ScalarKind , output : & mut dyn StringBuilder < 'i > , error : & mut dyn ErrorSink ,) -> ScalarKind { output . clear () ; let underscore = "_" ; if has_underscore (stream) { if stream . starts_with (underscore) { error . report_error (ParseError :: new ("`_` may only go between digits") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& []) . with_unexpected (Span :: new_unchecked (0 , underscore . len ())) ,) ; } if 1 < stream . len () && stream . ends_with (underscore) { let start = stream . offset_from (& raw . as_str ()) ; let end = start + stream . len () ; error . report_error (ParseError :: new ("`_` may only go between digits") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& []) . with_unexpected (Span :: new_unchecked (end - underscore . len () , end)) ,) ; } for part in stream . split (underscore) { let part_start = part . offset_from (& raw . as_str ()) ; let part_end = part_start + part . len () ; if 0 < part_start { let first = part . as_bytes () . first () . copied () . unwrap_or (b'0') ; if ! is_any_digit (first , kind) { let start = part_start - 1 ; let end = part_start ; debug_assert_eq ! (& raw . as_str () [start .. end] , underscore) ; error . report_error (ParseError :: new ("`_` may only go between digits") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_unexpected (Span :: new_unchecked (start , end)) ,) ; } } if 1 < part . len () && part_end < raw . len () { let last = part . as_bytes () . last () . copied () . unwrap_or (b'0') ; if ! is_any_digit (last , kind) { let start = part_end ; let end = start + underscore . len () ; debug_assert_eq ! (& raw . as_str () [start .. end] , underscore) ; error . report_error (ParseError :: new ("`_` may only go between digits") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_unexpected (Span :: new_unchecked (start , end)) ,) ; } } if part . is_empty () && part_start != 0 && part_end != raw . len () { let start = part_start ; let end = start + 1 ; error . report_error (ParseError :: new ("`_` may only go between digits") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_unexpected (Span :: new_unchecked (start , end)) ,) ; } if ! part . is_empty () && ! output . push_str (part) { error . report_error (ParseError :: new (ALLOCATION_ERROR) . with_unexpected (Span :: new_unchecked (part_start , part_end)) ,) ; } } } else { if ! output . push_str (stream) { error . report_error (ParseError :: new (ALLOCATION_ERROR) . with_unexpected (Span :: new_unchecked (0 , raw . len ())) ,) ; } } kind }
    };
}

decode_float_or_integer!()