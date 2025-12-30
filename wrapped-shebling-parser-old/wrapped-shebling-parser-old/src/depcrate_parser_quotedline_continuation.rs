// Generated macro for line_continuation (function)
macro_rules! Depcrate_parser_quotedline_continuation {
() => {
// Module: crate::parser::quoted
// Provides: {"line_continuation"}
// Dependencies: {}
pub (super) fn line_continuation (span : Span) -> ParseResult < () > { swallow (pair (backslash , newline)) (span) }
};
}
