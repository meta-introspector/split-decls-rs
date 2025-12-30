// Generated macro for err_unexpected_token (function)
macro_rules! Depcrate_parseerr_unexpected_token {
() => {
// Module: crate::parse
// Provides: {"err_unexpected_token"}
// Dependencies: {}
fn err_unexpected_token (span : Span , delimiter : Delimiter) -> Error { let msg = match delimiter { Delimiter :: Parenthesis => "unexpected token, expected `)`" , Delimiter :: Brace => "unexpected token, expected `}`" , Delimiter :: Bracket => "unexpected token, expected `]`" , Delimiter :: None => "unexpected token" , } ; Error :: new (span , msg) }
};
}
