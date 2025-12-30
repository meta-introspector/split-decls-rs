// Generated macro for lex_comment (function)
macro_rules! Depcrate_lexerlex_comment {
() => {
// Module: crate::lexer
// Provides: {"lex_comment"}
// Dependencies: {}
# [doc = " Process Comment"] # [doc = ""] # [doc = " ```bnf"] # [doc = " ;; Comment"] # [doc = ""] # [doc = " comment-start-symbol = %x23 ; #"] # [doc = " non-ascii = %x80-D7FF / %xE000-10FFFF"] # [doc = " non-eol = %x09 / %x20-7F / non-ascii"] # [doc = ""] # [doc = " comment = comment-start-symbol *non-eol"] # [doc = " ```"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `stream` must be UTF-8"] # [doc = " - `stream[0] == b'#'`"] fn lex_comment (stream : & mut Stream < '_ >) -> Token { let start = stream . current_token_start () ; let offset = stream . as_bytes () . find_slice ((b'\r' , b'\n')) . map (| s | s . start) . unwrap_or_else (| | stream . eof_offset ()) ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; let end = stream . previous_token_end () ; let span = Span :: new_unchecked (start , end) ; Token :: new (TokenKind :: Comment , span) }
};
}
