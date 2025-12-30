// Generated macro for last_tok (function)
macro_rules! Depcrate_macroslast_tok {
() => {
// Module: crate::macros
// Provides: {"last_tok"}
// Dependencies: {}
fn last_tok (tt : & TokenTree) -> Token { match * tt { TokenTree :: Token (ref t , _) => t . clone () , TokenTree :: Delimited (delim_span , _ , delim , _) => Token { kind : TokenKind :: CloseDelim (delim) , span : delim_span . close , } , } }
};
}
