// Generated macro for whitespace (function)
macro_rules! Depcrate_parser_triviawhitespace {
() => {
// Module: crate::parser::trivia
// Provides: {"whitespace"}
// Dependencies: {}
pub (super) fn whitespace (span : ParseSpan) -> ParseResult < char > { alt ((line_space , carriage_return , line_ending)) (span) }
};
}
