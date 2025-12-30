// Generated macro for pipeline (function)
macro_rules! Depcrate_parser_commandpipeline {
() => {
// Module: crate::parser::command
// Provides: {"pipeline"}
// Dependencies: {}
fn pipeline (span : Span) -> ParseResult < Pipeline > { let (span , _) = context ("unexpected token" , not (peek_sus_token)) (span) ? ; let (span , _) = opt (| span | { let (span , trivia) = preceded (char ('!') , trivia) (span) ? ; if trivia . is_empty () { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: MissingSpace) . range (& span)) ; } Ok ((span , ())) }) (span) ? ; let (span , cmds) = separated_list1 (context ("invalid pipeline operator!" , delimited (not (token (ControlOp :: OrIf)) , alt ((token (ControlOp :: OrAnd) , token (ControlOp :: Or))) , pair (trivia , linebreak) ,) ,) , cmd ,) (span) ? ; let (span , _) = trivia (span) ? ; Ok ((span , Pipeline :: new (cmds))) }
};
}
