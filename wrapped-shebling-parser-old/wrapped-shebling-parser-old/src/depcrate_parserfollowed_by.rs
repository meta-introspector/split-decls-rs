// Generated macro for followed_by (function)
macro_rules! Depcrate_parserfollowed_by {
() => {
// Module: crate::parser
// Provides: {"followed_by"}
// Dependencies: {}
fn followed_by < 'a , P , R > (parser : P) -> impl FnMut (Span < 'a >) -> ParseResult < bool > where P : FnMut (Span < 'a >) -> ParseResult < R > , { map (opt (peek (parser)) , | res | res . is_some ()) }
};
}
