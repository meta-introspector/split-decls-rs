// Generated macro for cond_cmd (function)
macro_rules! Depcrate_parser_commandcond_cmd {
() => {
// Module: crate::parser::command
// Provides: {"cond_cmd"}
// Dependencies: {}
fn cond_cmd (span : Span) -> ParseResult < CompoundCmd > { let (span , (cond , redirs)) = pair (cond , many0 (redir)) (span) ? ; let (span , cond_op) = opt (peek (ranged (consumed (alt ((value (ControlOp :: AndIf , alt ((tag ("-a") , tag ("and")))) , value (ControlOp :: OrIf , alt ((tag ("-o") , tag ("or")))) ,)))))) (span) ? ; if let Some (((bad_op , good_op) , range)) = & cond_op { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: BadOperator) . range (* range) . help (format ! ("Use {} instead of {} between test commands." , good_op . token () , bad_op)) ,) ; } let (span , (sus_token , next_word)) = pair (opt (peek_sus_token) , opt (peek (ranged (word)))) (span) ? ; if let Some ((_ , range)) = next_word { if sus_token . is_none () && cond_op . is_none () { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: SusToken) . label ("unexpected parameter after condition" , range) . help ("You might be missing a && or ||.") ,) } } Ok ((span , CompoundCmd :: new (cond , redirs))) }
};
}
