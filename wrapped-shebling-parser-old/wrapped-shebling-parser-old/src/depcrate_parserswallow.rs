// Generated macro for swallow (function)
macro_rules! Depcrate_parserswallow {
() => {
// Module: crate::parser
// Provides: {"swallow"}
// Dependencies: {}
fn swallow < 'a , P , R > (parser : P) -> impl FnMut (Span < 'a >) -> ParseResult < () > where P : FnMut (Span < 'a >) -> ParseResult < R > , { value (() , parser) }
};
}
