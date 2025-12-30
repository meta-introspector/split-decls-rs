// Generated macro for carriage_return (function)
macro_rules! Depcrate_parser_triviacarriage_return {
() => {
// Module: crate::parser::trivia
// Provides: {"carriage_return"}
// Dependencies: {}
fn carriage_return (span : ParseSpan) -> ParseResult < char > { let (span , cr) = spanned (char ('\r')) (span) ? ; span . diag (Diagnostic :: builder (DiagnosticKind :: CarrigeReturn) . span (cr)) ; Ok ((span , '\r')) }
};
}
