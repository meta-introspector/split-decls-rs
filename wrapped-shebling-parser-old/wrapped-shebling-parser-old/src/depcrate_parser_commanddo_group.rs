// Generated macro for do_group (function)
macro_rules! Depcrate_parser_commanddo_group {
() => {
// Module: crate::parser::command
// Provides: {"do_group"}
// Dependencies: {}
fn do_group (span : Span) -> ParseResult < Term > { let (span , _) = context ("did you forget the 'do' for this loop?" , not (token (Keyword :: Done)) ,) (span) ? ; let (span , _) = terminated (context ("expected 'do'!" , token (Keyword :: Do)) , trivia) (span) ? ; let (span , semi) = terminated (opt (ranged (token (ControlOp :: Semi))) , multi_trivia) (span) ? ; if let Some ((_ , range)) = semi { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: BadOperator) . label ("semicolon after 'do'" , range) . help ("Just delete it.") ,) ; } let (span , term) = preceded (context ("empty 'do' clause!" , not (token (Keyword :: Done))) , term ,) (span) ? ; let (span , _) = terminated (context ("expected 'done'!" , token (Keyword :: Done)) , trivia) (span) ? ; let (span , proc_sub_start) = opt (peek (ranged (tag ("<(")))) (span) ? ; if let Some ((_ , range)) = proc_sub_start { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: SusToken) . label ("process substitution?" , range) . help ("For redirection, use 'done < <(cmd)' (you're missing one '<').") ,) ; } Ok ((span , term)) }
};
}
