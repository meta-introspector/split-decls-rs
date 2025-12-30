// Generated macro for ranged (function)
macro_rules! Depcrate_parserranged {
() => {
// Module: crate::parser
// Provides: {"ranged"}
// Dependencies: {}
fn ranged < 'a , P , R > (parser : P) -> impl FnMut (Span < 'a >) -> ParseResult < (R , Range) > where P : FnMut (Span < 'a >) -> ParseResult < R > , { map (tuple ((position , parser , position)) , | (start , res , end) | { (res , Range :: new (start , end)) }) }
};
}
