macro_rules! deps {
    () => {
        Stream!();
        Token!();
        Span!();
        TokenKind!();
    };
}

macro_rules! lex_basic_string {
    () => {
        deps!();
        # [doc = " Process basic string"] # [doc = ""] # [doc = " ```bnf"] # [doc = " ;; Basic String"] # [doc = ""] # [doc = " basic-string = quotation-mark *basic-char quotation-mark"] # [doc = ""] # [doc = " quotation-mark = %x22            ; \""] # [doc = ""] # [doc = " basic-char = basic-unescaped / escaped"] # [doc = " basic-unescaped = wschar / %x21 / %x23-5B / %x5D-7E / non-ascii"] # [doc = " escaped = escape escape-seq-char"] # [doc = ""] # [doc = " escape = %x5C                   ; \\"] # [doc = " escape-seq-char =  %x22         ; \"    quotation mark  U+0022"] # [doc = " escape-seq-char =/ %x5C         ; \\    reverse solidus U+005C"] # [doc = " escape-seq-char =/ %x62         ; b    backspace       U+0008"] # [doc = " escape-seq-char =/ %x66         ; f    form feed       U+000C"] # [doc = " escape-seq-char =/ %x6E         ; n    line feed       U+000A"] # [doc = " escape-seq-char =/ %x72         ; r    carriage return U+000D"] # [doc = " escape-seq-char =/ %x74         ; t    tab             U+0009"] # [doc = " escape-seq-char =/ %x75 4HEXDIG ; uXXXX                U+XXXX"] # [doc = " escape-seq-char =/ %x55 8HEXDIG ; UXXXXXXXX            U+XXXXXXXX"] # [doc = " ```"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `stream` must be UTF-8"] # [doc = " - `stream[0] == b'\"'`"] fn lex_basic_string (stream : & mut Stream < '_ >) -> Token { let start = stream . current_token_start () ; let offset = 1 ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; loop { match stream . as_bstr () . find_slice ((QUOTATION_MARK , ESCAPE , b'\n')) { Some (span) => { let found = stream . as_bstr () [span . start] ; if found == QUOTATION_MARK { let offset = span . end ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; break ; } else if found == ESCAPE { let offset = span . end ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; let peek = stream . as_bstr () . peek_token () ; match peek { Some (ESCAPE) | Some (QUOTATION_MARK) => { let offset = 1 ; # [cfg (feature = "unsafe")] # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; } _ => { } } continue ; } else if found == b'\n' { let offset = span . start ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; break ; } else { unreachable ! ("found `{found}`") ; } } None => { stream . finish () ; break ; } } } let end = stream . previous_token_end () ; let span = Span :: new_unchecked (start , end) ; Token :: new (TokenKind :: BasicString , span) }
    };
}

lex_basic_string!();