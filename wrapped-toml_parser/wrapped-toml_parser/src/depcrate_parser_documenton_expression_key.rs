// Generated macro for on_expression_key (function)
macro_rules! Depcrate_parser_documenton_expression_key {
() => {
// Module: crate::parser::document
// Provides: {"on_expression_key"}
// Dependencies: {}
# [doc = " Start an expression from a key compatible token  type"] # [doc = ""] # [doc = " ```abnf"] # [doc = " expression =  ws [ comment ]"] # [doc = " expression =/ ws keyval ws [ comment ]"] # [doc = " expression =/ ws table ws [ comment ]"] # [doc = ""] # [doc = " ;; Key-Value pairs"] # [doc = ""] # [doc = " keyval = key keyval-sep val"] # [doc = " ```"] fn on_expression_key < 'i > (tokens : & mut Stream < 'i > , key_token : & 'i Token , encoding : Option < Encoding > , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink ,) { receiver . simple_key (key_token . span () , encoding , error) ; opt_dot_keys (tokens , receiver , error) ; opt_whitespace (tokens , receiver , error) ; let Some (eq_token) = next_token_if (tokens , | k | matches ! (k , TokenKind :: Equals)) else { if let Some (peek_token) = tokens . first () { let span = peek_token . span () . before () ; error . report_error (ParseError :: new ("key with no value") . with_context (span) . with_expected (& [Expected :: Literal ("=")]) . with_unexpected (span) ,) ; } ignore_to_newline (tokens , receiver , error) ; return ; } ; on_expression_key_val_sep (tokens , eq_token , receiver , error) ; }
};
}
