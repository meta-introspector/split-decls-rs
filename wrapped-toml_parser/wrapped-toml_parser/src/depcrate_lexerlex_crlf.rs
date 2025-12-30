// Generated macro for lex_crlf (function)
macro_rules! Depcrate_lexerlex_crlf {
() => {
// Module: crate::lexer
// Provides: {"lex_crlf"}
// Dependencies: {}
# [doc = " Process Newline"] # [doc = ""] # [doc = " ```bnf"] # [doc = " ;; Newline"] # [doc = ""] # [doc = " newline =  %x0A     ; LF"] # [doc = " newline =/ %x0D.0A  ; CRLF"] # [doc = " ```"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `stream` must be UTF-8"] # [doc = " - `stream[0] == b'\\r'`"] fn lex_crlf (stream : & mut Stream < '_ >) -> Token { let start = stream . current_token_start () ; let mut offset = '\r' . len_utf8 () ; let has_lf = stream . as_bstr () . get (1) == Some (& b'\n') ; if has_lf { offset += '\n' . len_utf8 () ; } # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; let end = stream . previous_token_end () ; let span = Span :: new_unchecked (start , end) ; Token :: new (TokenKind :: Newline , span) }
};
}
