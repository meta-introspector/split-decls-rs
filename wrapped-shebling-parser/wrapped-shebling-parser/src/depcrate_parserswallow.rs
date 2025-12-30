// Generated macro for swallow (function)
macro_rules! Depcrate_parserswallow {
() => {
// Module: crate::parser
// Provides: {"swallow"}
// Dependencies: {}
fn swallow < 'a , P , R > (parser : P) -> impl FnMut (ParseSpan < 'a >) -> ParseResult < () > where P : FnMut (ParseSpan < 'a >) -> ParseResult < R > , { value (() , parser) }
};
}
