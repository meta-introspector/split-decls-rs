macro_rules! deps {
    () => {
        TokenKind!();
        Stream!();
        Span!();
        Token!();
    };
}

macro_rules! lex_ml_basic_string {
    () => {
        deps!();
        # [doc = " Process multi-line basic string"] # [doc = ""] # [doc = " ```bnf"] # [doc = " ;; Multiline Basic String"] # [doc = ""] # [doc = " ml-basic-string = ml-basic-string-delim [ newline ] ml-basic-body"] # [doc = "                   ml-basic-string-delim"] # [doc = " ml-basic-string-delim = 3quotation-mark"] # [doc = " ml-basic-body = *mlb-content *( mlb-quotes 1*mlb-content ) [ mlb-quotes ]"] # [doc = ""] # [doc = " mlb-content = mlb-char / newline / mlb-escaped-nl"] # [doc = " mlb-char = mlb-unescaped / escaped"] # [doc = " mlb-quotes = 1*2quotation-mark"] # [doc = " mlb-unescaped = wschar / %x21 / %x23-5B / %x5D-7E / non-ascii"] # [doc = " mlb-escaped-nl = escape ws newline *( wschar / newline )"] # [doc = " ```"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `stream` must be UTF-8"] # [doc = " - `stream.starts_with(ML_BASIC_STRING_DELIM)`"] fn lex_ml_basic_string (stream : & mut Stream < '_ >) -> Token { let start = stream . current_token_start () ; let offset = ML_BASIC_STRING_DELIM . len () ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; loop { match stream . as_bstr () . find_slice ((ML_BASIC_STRING_DELIM , "\\")) { Some (span) => { let found = stream . as_bstr () [span . start] ; if found == QUOTATION_MARK { let offset = span . end ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; break ; } else if found == ESCAPE { let offset = span . end ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; let peek = stream . as_bstr () . peek_token () ; match peek { Some (ESCAPE) | Some (QUOTATION_MARK) => { let offset = 1 ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; } _ => { } } continue ; } else { unreachable ! ("found `{found}`") ; } } None => { stream . finish () ; break ; } } } if stream . as_bstr () . peek_token () == Some (QUOTATION_MARK) { let offset = 1 ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; if stream . as_bstr () . peek_token () == Some (QUOTATION_MARK) { let offset = 1 ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; } } let end = stream . previous_token_end () ; let span = Span :: new_unchecked (start , end) ; Token :: new (TokenKind :: MlBasicString , span) }
    };
}

lex_ml_basic_string!()