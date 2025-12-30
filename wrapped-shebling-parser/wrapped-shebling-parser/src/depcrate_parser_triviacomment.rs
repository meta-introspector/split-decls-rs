// Generated macro for comment (function)
macro_rules! Depcrate_parser_triviacomment {
() => {
// Module: crate::parser::trivia
// Provides: {"comment"}
// Dependencies: {}
fn comment (span : ParseSpan) -> ParseResult < String > { recognize_string (pair (char ('#') , is_not ("\r\n"))) (span) }
};
}
