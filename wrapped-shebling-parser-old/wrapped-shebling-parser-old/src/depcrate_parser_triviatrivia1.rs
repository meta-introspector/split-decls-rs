// Generated macro for trivia1 (function)
macro_rules! Depcrate_parser_triviatrivia1 {
() => {
// Module: crate::parser::trivia
// Provides: {"trivia1"}
// Dependencies: {}
pub (super) fn trivia1 (span : Span) -> ParseResult < String > { context ("expected whitespace!" , verify (trivia , | trivia : & str | ! trivia . is_empty ()) ,) (span) }
};
}
