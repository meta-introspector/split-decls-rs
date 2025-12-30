// Generated macro for dollar_var (function)
macro_rules! Depcrate_parser_expansiondollar_var {
() => {
// Module: crate::parser::expansion
// Provides: {"dollar_var"}
// Dependencies: {}
fn dollar_var (span : Span) -> ParseResult < String > { fn dollar_ident (span : Span) -> ParseResult < String > { let (span , ((ident , range) , has_bracket)) = pair (ranged (identifier) , followed_by (char ('['))) (span) ? ; if has_bracket { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: Unbraced) . label ("use braces when expanding arrays" , range) . help (format ! ("Use ${{{}[..]}} to tell the shell that the square brackets are part of the expansion." , ident))) ; } Ok ((span , ident)) } fn dollar_digit (span : Span) -> ParseResult < String > { let (span , (start , dig)) = pair (position , satisfy (| c | c . is_ascii_digit ())) (span) ? ; let (span , extra) = opt (peek (pair (digit1 , position))) (span) ? ; if let Some ((extra_digs , end)) = extra { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: Unbraced) . label ("braces are required for multi-digit positionals" , Range :: new (start , end)) . help (format ! ("${0}{1} is interpreted as ${0} followed by a literal {1}. Use ${{{0}{1}}} instead." , dig , extra_digs)) ,) ; } Ok ((span , dig . into ())) } preceded (char ('$') , alt ((dollar_digit , recognize_string (one_of (SPECIAL_PARAMS)) , dollar_ident ,)) ,) (span) }
};
}
