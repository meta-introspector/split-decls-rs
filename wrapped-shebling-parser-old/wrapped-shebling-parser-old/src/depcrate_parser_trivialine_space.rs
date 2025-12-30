// Generated macro for line_space (function)
macro_rules! Depcrate_parser_trivialine_space {
() => {
// Module: crate::parser::trivia
// Provides: {"line_space"}
// Dependencies: {}
pub (super) fn line_space (span : Span) -> ParseResult < char > { alt ((one_of (" \t") , | span | { let (span , (_ , range)) = ranged (one_of (UNISPACES)) (span) ? ; span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: Unichar) . label ("unicode space" , range) ,) ; Ok ((span , ' ')) })) (span) }
};
}
