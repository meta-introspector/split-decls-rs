// Generated macro for spanned (function)
macro_rules! Depcrate_parserspanned {
() => {
// Module: crate::parser
// Provides: {"spanned"}
// Dependencies: {}
fn spanned < 'a , P , R > (parser : P) -> impl FnMut (ParseSpan < 'a >) -> ParseResult < Spanned < R > > where P : FnMut (ParseSpan < 'a >) -> ParseResult < R > , { map (tuple ((offset , parser , offset)) , | (start , res , end) | { Spanned :: new (res , Span :: new (start , end)) }) }
};
}
