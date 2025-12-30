// Generated macro for line_space (function)
macro_rules! Depcrate_parser_trivialine_space {
() => {
// Module: crate::parser::trivia
// Provides: {"line_space"}
// Dependencies: {}
pub (super) fn line_space (span : ParseSpan) -> ParseResult < char > { alt ((one_of (" \t") , | span | { let (span , unispace) = spanned (one_of (UNISPACES)) (span) ? ; span . diag (Diagnostic :: builder (DiagnosticKind :: Unichar) . label ("unicode space" , unispace)) ; Ok ((span , ' ')) })) (span) }
};
}
