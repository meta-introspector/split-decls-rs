// Generated macro for single_quoted (function)
macro_rules! Depcrate_parser_quotedsingle_quoted {
() => {
// Module: crate::parser::quoted
// Provides: {"single_quoted"}
// Dependencies: {}
pub (super) fn single_quoted (span : ParseSpan) -> ParseResult < String > { let (span , string) = preceded (char ('\'') , recognize_string (take_till (| c | c == '\''))) (span) ? ; let last_char = string . chars () . last () . unwrap_or_default () ; if last_char == '\\' { span . diag (Diagnostic :: builder (DiagnosticKind :: BadEscape) . label ("the backslash before this quote is literal" , span . offset ()) . help ("Wanna escape a single quote? 'Let'\\''s do it correctly'") ,) ; } let (span , quote) = spanned (cut (context ("expected ending single quote!" , char ('\'')))) (span) ? ; let (span , alphabetic_follows) = peeked (satisfy (| c | c . is_ascii_alphabetic ())) (span) ? ; if alphabetic_follows && last_char . is_ascii_alphabetic () { span . diag (Diagnostic :: builder (DiagnosticKind :: BadQuote) . label ("this apostrophe terminates the string!" , quote) . help ("Try escaping the apostrophe, 'it'\\''s done like this!'") ,) ; } Ok ((span , string)) }
};
}
