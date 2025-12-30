// Generated macro for for_loop (function)
macro_rules! Depcrate_parser_commandfor_loop {
() => {
// Module: crate::parser::command
// Provides: {"for_loop"}
// Dependencies: {}
fn for_loop (span : Span) -> ParseResult < ForLoop > { fn arith_for_loop (span : Span) -> ParseResult < ArithForLoop > { let (span , header) = delimited (arith_parens ('(' , "missing the second ( opening this arithmetic for-loop!" ,) , tuple ((terminated (arith_seq , pair (char (';') , trivia)) , terminated (arith_seq , pair (char (';') , trivia)) , terminated (arith_seq , trivia) ,)) , arith_parens (')' , "missing the second ) closing this arithmetic for-loop!" ,) ,) (span) ? ; let (span , _) = delimited (trivia , opt (alt ((preceded (token (ControlOp :: Semi) , linebreak) , newline_list ,))) , trivia ,) (span) ? ; let (span , body) = alt ((brace_group , do_group)) (span) ? ; Ok ((span , ArithForLoop :: new (header , body))) } fn arith_parens (paren : char , ctx_msg : & 'static str) -> impl Fn (Span) -> ParseResult < () > { move | span | { let (span , trivia) = delimited (char (paren) , opt (ranged (trivia1)) , context (ctx_msg , char (paren)) ,) (span) ? ; if let Some ((_ , range)) = trivia { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: BadSpace) . range (range) . help (format ! ("Remove spacing between '{0}{0}' in this arithmetic for-loop." , paren)) ,) ; } Ok ((span , ())) } } preceded (pair (token (Keyword :: For) , trivia) , alt ((into (arith_for_loop) , into (in_listed))) ,) (span) }
};
}
