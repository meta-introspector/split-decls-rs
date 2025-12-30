// Generated macro for peeked (function)
macro_rules! Depcrate_parserpeeked {
() => {
// Module: crate::parser
// Provides: {"peeked"}
// Dependencies: {}
fn peeked < 'a , P , R > (parser : P) -> impl FnMut (ParseSpan < 'a >) -> ParseResult < bool > where P : Fn (ParseSpan < 'a >) -> ParseResult < R > , { map (opt (peek (parser)) , | res | res . is_some ()) }
};
}
