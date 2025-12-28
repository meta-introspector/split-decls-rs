macro_rules! deps {
    () => {
        ParseError!();
        Expected!();
        Span!();
        Raw!();
        ErrorSink!();
    };
}

macro_rules! decode_newline {
    () => {
        deps!();
        # [doc = " Parse newline"] # [doc = ""] # [doc = " ```bnf"] # [doc = ";; Newline"] # [doc = ""] # [doc = " newline =  %x0A     ; LF"] # [doc = " newline =/ %x0D.0A  ; CRLF"] # [doc = " ```"] pub (crate) fn decode_newline (raw : Raw < '_ > , error : & mut dyn ErrorSink) { let s = raw . as_str () ; if s == "\r" { error . report_error (ParseError :: new ("carriage return must be followed by newline") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Literal ("\n")]) . with_unexpected (Span :: new_unchecked (raw . len () , raw . len ())) ,) ; } }
    };
}

decode_newline!()