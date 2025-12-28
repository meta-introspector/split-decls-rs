macro_rules! deps {
    () => {
        Raw!();
        ErrorSink!();
        Span!();
        Expected!();
        ParseError!();
    };
}

macro_rules! hexescape {
    () => {
        deps!();
        fn hexescape (stream : & mut & str , num_digits : usize , raw : Raw < '_ > , error : & mut dyn ErrorSink ,) -> char { let offset = stream . as_bytes () . offset_for (| b | ! HEXDIG . contains_token (b)) . unwrap_or_else (| | stream . eof_offset ()) . min (num_digits) ; # [cfg (feature = "unsafe")] let value = unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] let value = stream . next_slice (offset) ; if value . len () != num_digits { let offset = stream . offset_from (& raw . as_str ()) ; error . report_error (ParseError :: new ("too few unicode value digits") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Description ("unicode hexadecimal value")]) . with_unexpected (Span :: new_unchecked (offset , offset)) ,) ; return '�' ; } let Some (value) = u32 :: from_str_radix (value , 16) . ok () . and_then (char :: from_u32) else { let offset = value . offset_from (& raw . as_str ()) ; error . report_error (ParseError :: new ("invalid value") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Description ("unicode hexadecimal value")]) . with_unexpected (Span :: new_unchecked (offset , offset)) ,) ; return '�' ; } ; value }
    };
}

hexescape!();