// Generated macro for next_token_if (function)
macro_rules! Depcrate_parser_documentnext_token_if {
() => {
// Module: crate::parser::document
// Provides: {"next_token_if"}
// Dependencies: {}
fn next_token_if < 'i , F : Fn (TokenKind) -> bool > (tokens : & mut Stream < 'i > , pred : F ,) -> Option < & 'i Token > { match tokens . first () { Some (next) if pred (next . kind ()) => tokens . next_token () , _ => None , } }
};
}
