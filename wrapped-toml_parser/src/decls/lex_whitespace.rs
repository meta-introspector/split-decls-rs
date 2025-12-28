macro_rules! deps {
    () => {
        Stream!();
        Token!();
        Span!();
        TokenKind!();
    };
}

macro_rules! lex_whitespace {
    () => {
        deps!();
        # [doc = " Process Whitespace"] # [doc = ""] # [doc = " ```bnf"] # [doc = " ;; Whitespace"] # [doc = ""] # [doc = " ws = *wschar"] # [doc = " wschar =  %x20  ; Space"] # [doc = " wschar =/ %x09  ; Horizontal tab"] # [doc = " ```"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `stream` must be UTF-8"] # [doc = " - `stream` must be non-empty"] fn lex_whitespace (stream : & mut Stream < '_ >) -> Token { debug_assert ! (! stream . is_empty ()) ; let start = stream . current_token_start () ; let offset = stream . as_bstr () . offset_for (| b | ! WSCHAR . contains_token (b)) . unwrap_or (stream . eof_offset ()) ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; let end = stream . previous_token_end () ; let span = Span :: new_unchecked (start , end) ; Token :: new (TokenKind :: Whitespace , span) }
    };
}

lex_whitespace!()