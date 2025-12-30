// Generated macro for carriage_return (function)
macro_rules! Depcrate_parser_triviacarriage_return {
() => {
// Module: crate::parser::trivia
// Provides: {"carriage_return"}
// Dependencies: {}
fn carriage_return (span : Span) -> ParseResult < char > { let (span , (cr , range)) = ranged (char ('\r')) (span) ? ; span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: SusToken) . label ("carriage return" , range) . help ("Try running the script through tr -d '\\r'.") ,) ; Ok ((span , cr)) }
};
}
