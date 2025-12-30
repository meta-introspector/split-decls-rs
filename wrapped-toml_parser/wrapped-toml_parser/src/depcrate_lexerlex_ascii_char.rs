// Generated macro for lex_ascii_char (function)
macro_rules! Depcrate_lexerlex_ascii_char {
() => {
// Module: crate::lexer
// Provides: {"lex_ascii_char"}
// Dependencies: {}
# [doc = " Process an ASCII character token"] # [doc = ""] # [doc = " # Safety"] # [doc = ""] # [doc = " - `stream` must be UTF-8"] # [doc = " - `stream` must be non-empty"] # [doc = " - `stream[0]` must be ASCII"] fn lex_ascii_char (stream : & mut Stream < '_ > , kind : TokenKind) -> Token { debug_assert ! (! stream . is_empty ()) ; let start = stream . current_token_start () ; let offset = 1 ; # [cfg (feature = "unsafe")] unsafe { stream . next_slice_unchecked (offset) } ; # [cfg (not (feature = "unsafe"))] stream . next_slice (offset) ; let end = stream . previous_token_end () ; let span = Span :: new_unchecked (start , end) ; Token :: new (kind , span) }
};
}
