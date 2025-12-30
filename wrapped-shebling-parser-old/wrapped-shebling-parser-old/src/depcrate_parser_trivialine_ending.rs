// Generated macro for line_ending (function)
macro_rules! Depcrate_parser_trivialine_ending {
() => {
// Module: crate::parser::trivia
// Provides: {"line_ending"}
// Dependencies: {}
pub (super) fn line_ending (span : Span) -> ParseResult < char > { preceded (opt (carriage_return) , newline) (span) }
};
}
