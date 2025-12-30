// Generated macro for escaping_backquoted (function)
macro_rules! Depcrate_parser_quotedescaping_backquoted {
() => {
// Module: crate::parser::quoted
// Provides: {"escaping_backquoted"}
// Dependencies: {}
fn escaping_backquoted (escape_double_quotes : bool) -> impl Fn (Span) -> ParseResult < Term > { fn backquote (span : Span) -> ParseResult < () > { alt ((swallow (char ('`')) , | span | { let (span , (_ , range)) = ranged (char ('´')) (span) ? ; span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: SusToken) . label ("forward tick" , range) . help ("For command expansion, use backticks (``).") ,) ; Ok ((span , ())) })) (span) } move | span | { let (span , cmd_start) = preceded (backquote , position) (span) ? ; let (span , cmd) = many0 (alt ((escaped (if escape_double_quotes { "$`\\\"" } else { "$`\\" }) , recognize_string (is_not ("\\`´")) ,))) (span) ? ; let (span , _) = context ("expected ending backtick!" , backquote) (span) ? ; let cmd = cmd . concat () ; let cmd_span = unsafe { Span :: new_from_raw_offset (cmd_start . location_offset () , cmd_start . location_line () , & cmd , ParseContext :: new () ,) } ; let (cmd_span , cmd) = all_consuming (preceded (multi_trivia , term)) (cmd_span) ? ; span . extra . extend_diags (cmd_span . extra) ; Ok ((span , cmd)) } }
};
}
