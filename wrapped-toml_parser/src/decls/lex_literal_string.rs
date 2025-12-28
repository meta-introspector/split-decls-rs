macro_rules! deps {
    () => {
        Token!();
        Stream!();
        TokenKind!();
        Span!();
    };
}

macro_rules! lex_literal_string {
    () => {
        deps!();
        # [doc = " Process literal string"] # [doc = ""] # [doc = " ```bnf"] # [doc = " ;; Literal String"] # [doc = ""] # [doc = " literal-string = apostrophe *literal-char apostrophe"] # [doc = ""] # [doc = " apostrophe = %x27 ; ' apostrophe"] # [doc = ""] # [doc = " literal-char = %x09 / %x20-26 / %x28-7E / non-ascii"] # [doc = " ```"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `stream` must be UTF-8"] # [doc = " - `stream[0] == b'\\''`"] fn lex_literal_string (stream : & mut Stream < '_ >) -> Token { let start = stream . current_token_start () ; let offset = 1 ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; let offset = match stream . as_bstr () . find_slice ((APOSTROPHE , b'\n')) { Some (span) => { if stream . as_bstr () [span . start] == APOSTROPHE { span . end } else { span . start } } None => stream . eof_offset () , } ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; let end = stream . previous_token_end () ; let span = Span :: new_unchecked (start , end) ; Token :: new (TokenKind :: LiteralString , span) }
    };
}

lex_literal_string!()