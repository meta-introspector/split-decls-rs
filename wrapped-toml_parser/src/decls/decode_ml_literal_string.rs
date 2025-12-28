macro_rules! deps {
    () => {
        StringBuilder!();
        Expected!();
        ParseError!();
        Raw!();
        Span!();
        ErrorSink!();
    };
}

macro_rules! decode_ml_literal_string {
    () => {
        deps!();
        # [doc = " Parse multi-line literal string"] # [doc = ""] # [doc = " ```bnf"] # [doc = " ;; Multiline Literal String"] # [doc = ""] # [doc = " ml-literal-string = ml-literal-string-delim [ newline ] ml-literal-body"] # [doc = "                     ml-literal-string-delim"] # [doc = " ml-literal-string-delim = 3apostrophe"] # [doc = " ml-literal-body = *mll-content *( mll-quotes 1*mll-content ) [ mll-quotes ]"] # [doc = ""] # [doc = " mll-content = mll-char / newline"] # [doc = " mll-quotes = 1*2apostrophe"] # [doc = " ```"] pub (crate) fn decode_ml_literal_string < 'i > (raw : Raw < 'i > , output : & mut dyn StringBuilder < 'i > , error : & mut dyn ErrorSink ,) { const INVALID_STRING : & str = "invalid multi-line literal string" ; output . clear () ; let s = raw . as_str () ; let s = if let Some (stripped) = s . strip_prefix (ML_LITERAL_STRING_DELIM) { stripped } else { error . report_error (ParseError :: new (INVALID_STRING) . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Literal ("'")]) . with_unexpected (Span :: new_unchecked (0 , 0)) ,) ; s } ; let s = strip_start_newline (s) ; let s = if let Some (stripped) = s . strip_suffix (ML_LITERAL_STRING_DELIM) { stripped } else { error . report_error (ParseError :: new (INVALID_STRING) . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Literal ("'")]) . with_unexpected (Span :: new_unchecked (raw . len () , raw . len ())) ,) ; s . trim_end_matches ('\'') } ; for (i , b) in s . as_bytes () . iter () . enumerate () { if * b == b'\'' || * b == b'\n' { } else if * b == b'\r' { if s . as_bytes () . get (i + 1) != Some (& b'\n') { let offset = (& s . as_bytes () [i + 1 ..]) . offset_from (& raw . as_bytes ()) ; error . report_error (ParseError :: new ("carriage return must be followed by newline") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Literal ("\n")]) . with_unexpected (Span :: new_unchecked (offset , offset)) ,) ; } } else if ! MLL_CHAR . contains_token (b) { let offset = (& s . as_bytes () [i ..]) . offset_from (& raw . as_bytes ()) ; error . report_error (ParseError :: new (INVALID_STRING) . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Description ("non-single-quote characters")]) . with_unexpected (Span :: new_unchecked (offset , offset)) ,) ; } } if ! output . push_str (s) { error . report_error (ParseError :: new (ALLOCATION_ERROR) . with_unexpected (Span :: new_unchecked (0 , raw . len ())) ,) ; } }
    };
}

decode_ml_literal_string!()