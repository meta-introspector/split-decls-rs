// Generated macro for in_listed (function)
macro_rules! Depcrate_parser_commandin_listed {
() => {
// Module: crate::parser::command
// Provides: {"in_listed"}
// Dependencies: {}
fn in_listed (span : Span) -> ParseResult < InListed > { let (span , ((dollar , name) , range)) = terminated (ranged (pair (opt (char ('$')) , identifier)) , multi_trivia) (span) ? ; if dollar . is_some () { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: NotShellCode) . label ("dollar in iterator variable" , range) . help (format ! ("Simply use '{}'." , name)) ,) ; } let (span , list) = alt ((in_list , map (opt (alt ((preceded (pair (token (ControlOp :: Semi) , trivia) , linebreak) , newline_list ,))) , | _ | vec ! [] ,) ,)) (span) ? ; let (span , body) = alt ((brace_group , do_group)) (span) ? ; Ok ((span , InListed :: new (name , list , body))) }
};
}
