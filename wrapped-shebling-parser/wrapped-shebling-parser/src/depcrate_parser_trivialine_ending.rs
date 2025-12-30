// Generated macro for line_ending (function)
macro_rules! Depcrate_parser_trivialine_ending {
() => {
// Module: crate::parser::trivia
// Provides: {"line_ending"}
// Dependencies: {}
fn line_ending (span : ParseSpan) -> ParseResult < char > { preceded (opt (carriage_return) , newline) (span) }
};
}
