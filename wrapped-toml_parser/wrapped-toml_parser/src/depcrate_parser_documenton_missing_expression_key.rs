// Generated macro for on_missing_expression_key (function)
macro_rules! Depcrate_parser_documenton_missing_expression_key {
() => {
// Module: crate::parser::document
// Provides: {"on_missing_expression_key"}
// Dependencies: {}
# [cold] fn on_missing_expression_key (tokens : & mut Stream < '_ > , token : & Token , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink ,) { error . report_error (ParseError :: new ("invalid key-value pair") . with_context (token . span ()) . with_expected (& [Expected :: Description ("key")]) . with_unexpected (token . span () . before ()) ,) ; receiver . error (token . span () , error) ; ignore_to_newline (tokens , receiver , error) ; }
};
}
