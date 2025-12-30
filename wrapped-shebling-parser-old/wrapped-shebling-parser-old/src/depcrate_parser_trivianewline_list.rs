// Generated macro for newline_list (function)
macro_rules! Depcrate_parser_trivianewline_list {
() => {
// Module: crate::parser::trivia
// Provides: {"newline_list"}
// Dependencies: {}
pub (super) fn newline_list (span : Span) -> ParseResult < () > { let (span , _) = many1 (terminated (alt ((line_ending , carriage_return)) , trivia)) (span) ? ; let (span , op) = opt (peek (ranged (alt ((token (ControlOp :: AndIf) , token (ControlOp :: And) , token (ControlOp :: OrAnd) , token (ControlOp :: OrIf) , token (ControlOp :: Or) ,))))) (span) ? ; if let Some ((op , range)) = op { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: SusToken) . label ("control operator" , range) . help (format ! ("Move the {} to the end of the previous line." , op . token ())) ,) ; } Ok ((span , ())) }
};
}
