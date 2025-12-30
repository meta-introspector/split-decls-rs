// Generated macro for in_list (function)
macro_rules! Depcrate_parser_commandin_list {
() => {
// Module: crate::parser::command
// Provides: {"in_list"}
// Dependencies: {}
fn in_list (span : Span) -> ParseResult < Vec < Word > > { let (span , _) = pair (token (Keyword :: In) , trivia) (span) ? ; let (span , (list , _)) = many_till (terminated (word , trivia) , alt ((| span | { let (span , _) = peek (token (Keyword :: Do)) (span) ? ; span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: MissingSpace) . range (& span) . help ("Add a new line or semicolon before 'do'.") ,) ; Ok ((span , ())) } , terminated (alt ((swallow (token (ControlOp :: Semi)) , swallow (line_ending))) , multi_trivia ,) ,)) ,) (span) ? ; Ok ((span , list)) }
};
}
