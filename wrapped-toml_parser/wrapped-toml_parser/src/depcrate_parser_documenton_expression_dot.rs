// Generated macro for on_expression_dot (function)
macro_rules! Depcrate_parser_documenton_expression_dot {
() => {
// Module: crate::parser::document
// Provides: {"on_expression_dot"}
// Dependencies: {}
fn on_expression_dot < 'i > (tokens : & mut Stream < 'i > , dot_token : & 'i Token , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink ,) { receiver . simple_key (dot_token . span () . before () , None , error) ; seek (tokens , - 1) ; opt_dot_keys (tokens , receiver , error) ; opt_whitespace (tokens , receiver , error) ; let Some (eq_token) = next_token_if (tokens , | k | matches ! (k , TokenKind :: Equals)) else { if let Some (peek_token) = tokens . first () { let span = peek_token . span () . before () ; error . report_error (ParseError :: new ("missing value for key") . with_context (span) . with_expected (& [Expected :: Literal ("=")]) . with_unexpected (span) ,) ; } ignore_to_newline (tokens , receiver , error) ; return ; } ; on_expression_key_val_sep (tokens , eq_token , receiver , error) ; }
};
}
