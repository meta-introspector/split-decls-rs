// Generated macro for double_quoted_lit (function)
macro_rules! Depcrate_parser_quoteddouble_quoted_lit {
() => {
// Module: crate::parser::quoted
// Provides: {"double_quoted_lit"}
// Dependencies: {}
fn double_quoted_lit (span : Span) -> ParseResult < String > { fn double_quoted_escape (span : Span) -> ParseResult < String > { let (span , (escaped , range)) = ranged (escaped (DOUBLE_ESCAPABLE)) (span) ? ; if escaped . len () >= 2 { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: BadEscape) . label ("this character has no special behavior when escaped inside double quotes" , range ,) ,) ; } Ok ((span , escaped)) } fn lonely_dollar (span : Span) -> ParseResult < String > { let (span , (dollar , range)) = ranged (char ('$')) (span) ? ; let (span , next_char) = opt (peek (preceded (pair (char ('"') , opt (char ('"'))) , alt ((satisfy (| c | c . is_ascii_alphanumeric ()) , one_of ("_?!#-@"))) ,))) (span) ? ; if let Some (next_char) = next_char { span . extra . diag (ParseDiagnostic :: builder (ParseDiagnosticKind :: MissingEscape) . label ("is this supposed to be a literal dollar?" , range) . help (format ! ("Instead of \" .. $\"{0}, use \" .. \\${0} .. \"" , next_char)) ,) ; } Ok ((span , dollar . into ())) } alt ((map (many1 (alt ((double_quoted_escape , recognize_string (is_not (DOUBLE_ESCAPABLE)) ,))) , | sgmt | sgmt . concat () ,) , lonely_dollar ,)) (span) }
};
}
