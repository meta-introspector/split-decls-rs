// Generated macro for recognize_string (function)
macro_rules! Depcrate_parserrecognize_string {
() => {
// Module: crate::parser
// Provides: {"recognize_string"}
// Dependencies: {}
fn recognize_string < 'a , P , R > (parser : P) -> impl FnMut (ParseSpan < 'a >) -> ParseResult < String > where P : FnMut (ParseSpan < 'a >) -> ParseResult < R > , { map (recognize (parser) , | span | span . fragment () . into ()) }
};
}
