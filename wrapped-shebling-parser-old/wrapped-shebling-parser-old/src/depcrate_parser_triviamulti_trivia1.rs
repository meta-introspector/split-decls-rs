// Generated macro for multi_trivia1 (function)
macro_rules! Depcrate_parser_triviamulti_trivia1 {
() => {
// Module: crate::parser::trivia
// Provides: {"multi_trivia1"}
// Dependencies: {}
pub (super) fn multi_trivia1 (span : Span) -> ParseResult < String > { context ("expected whitespace!" , verify (multi_trivia , | trivia : & String | ! trivia . is_empty ()) ,) (span) }
};
}
