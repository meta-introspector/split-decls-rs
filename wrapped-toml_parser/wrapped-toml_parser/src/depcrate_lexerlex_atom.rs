// Generated macro for lex_atom (function)
macro_rules! Depcrate_lexerlex_atom {
() => {
// Module: crate::lexer
// Provides: {"lex_atom"}
// Dependencies: {}
# [doc = " Process Atom"] # [doc = ""] # [doc = " This is everything else"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `stream` must be UTF-8"] # [doc = " - `stream` must be non-empty"] fn lex_atom (stream : & mut Stream < '_ >) -> Token { let start = stream . current_token_start () ; const TOKEN_START : & [u8] = b".=,[]{} \t#\r\n" ; let offset = stream . as_bstr () . offset_for (| b | TOKEN_START . contains_token (b)) . unwrap_or_else (| | stream . eof_offset ()) ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; let end = stream . previous_token_end () ; let span = Span :: new_unchecked (start , end) ; Token :: new (TokenKind :: Atom , span) }
};
}
