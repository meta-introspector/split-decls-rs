macro_rules! deps {
    () => {
        ErrorSink!();
        Expected!();
        Raw!();
        ParseError!();
        Span!();
    };
}

macro_rules! mlb_escaped_nl {
    () => {
        deps!();
        # [doc = " ```bnf"] # [doc = " mlb-escaped-nl = escape ws newline *( wschar / newline )"] # [doc = " ```"] fn mlb_escaped_nl (stream : & mut & str , raw : Raw < '_ > , error : & mut dyn ErrorSink) { const INVALID_STRING : & str = "invalid multi-line basic string" ; let ws_offset = stream . as_bytes () . offset_for (| b | ! WSCHAR . contains_token (b)) . unwrap_or (stream . len ()) ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (ws_offset) ; } # [cfg (not (feature = "unsafe"))] stream . next_slice (ws_offset) ; let start = stream . checkpoint () ; match stream . next_token () { Some ('\n') => { } Some ('\r') => { if stream . as_bytes () . first () == Some (& b'\n') { let _ = stream . next_token () ; } else { let start = stream . offset_from (& raw . as_str ()) ; let end = start ; error . report_error (ParseError :: new ("carriage return must be followed by newline") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Literal ("\n")]) . with_unexpected (Span :: new_unchecked (start , end)) ,) ; } } _ => { stream . reset (& start) ; let start = stream . offset_from (& raw . as_str ()) ; let end = start ; error . report_error (ParseError :: new (INVALID_STRING) . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Literal ("\n")]) . with_unexpected (Span :: new_unchecked (start , end)) ,) ; } } loop { let start_offset = stream . offset_from (& raw . as_str ()) ; let offset = stream . as_bytes () . offset_for (| b | ! (WSCHAR , b'\n') . contains_token (b)) . unwrap_or (stream . len ()) ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) ; } # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; if stream . starts_with ("\r") { let offset = if stream . starts_with ("\r\n") { "\r\n" . len () } else { let start = stream . offset_from (& raw . as_str ()) + 1 ; error . report_error (ParseError :: new ("carriage return must be followed by newline") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Literal ("\n")]) . with_unexpected (Span :: new_unchecked (start , start)) ,) ; "\r" . len () } ; # [cfg (feature = "unsafe")] let _ = unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] let _ = stream . next_slice (offset) ; } let end_offset = stream . offset_from (& raw . as_str ()) ; if start_offset == end_offset { break ; } } }
    };
}

mlb_escaped_nl!();