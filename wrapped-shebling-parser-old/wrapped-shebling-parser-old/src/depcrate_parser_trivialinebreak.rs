// Generated macro for linebreak (function)
macro_rules! Depcrate_parser_trivialinebreak {
() => {
// Module: crate::parser::trivia
// Provides: {"linebreak"}
// Dependencies: {}
pub (super) fn linebreak (span : Span) -> ParseResult < () > { swallow (opt (newline_list)) (span) }
};
}
