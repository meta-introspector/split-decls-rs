macro_rules! deps {
    () => {
        ParseError!();
        Raw!();
        ErrorSink!();
        Span!();
        Expected!();
    };
}

macro_rules! decode_comment {
    () => {
        deps!();
        # [doc = " Parse comment"] # [doc = ""] # [doc = " ```bnf"] # [doc = " ;; Comment"] # [doc = ""] # [doc = " comment-start-symbol = %x23 ; #"] # [doc = " non-ascii = %x80-D7FF / %xE000-10FFFF"] # [doc = " non-eol = %x09 / %x20-7F / non-ascii"] # [doc = ""] # [doc = " comment = comment-start-symbol *non-eol"] # [doc = " ```"] pub (crate) fn decode_comment (raw : Raw < '_ > , error : & mut dyn ErrorSink) { let s = raw . as_bytes () ; if s . first () != Some (& COMMENT_START_SYMBOL) { error . report_error (ParseError :: new ("missing comment start") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Literal ("#")]) . with_unexpected (Span :: new_unchecked (0 , 0)) ,) ; } for (i , b) in s . iter () . copied () . enumerate () { if ! NON_EOL . contains_token (b) { error . report_error (ParseError :: new ("invalid comment character") . with_context (Span :: new_unchecked (0 , raw . len ())) . with_expected (& [Expected :: Description ("printable characters")]) . with_unexpected (Span :: new_unchecked (i , i)) ,) ; } } }
    };
}

decode_comment!();