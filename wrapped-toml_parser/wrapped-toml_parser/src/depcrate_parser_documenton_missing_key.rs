// Generated macro for on_missing_key (function)
macro_rules! Depcrate_parser_documenton_missing_key {
() => {
// Module: crate::parser::document
// Provides: {"on_missing_key"}
// Dependencies: {}
# [cold] fn on_missing_key (tokens : & mut Stream < '_ > , token : & Token , invalid_description : & 'static str , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink ,) { error . report_error (ParseError :: new (invalid_description) . with_context (token . span ()) . with_expected (& [Expected :: Description ("key")]) . with_unexpected (token . span () . before ()) ,) ; if token . kind () == TokenKind :: Eof { } else if token . kind () == TokenKind :: Newline { receiver . newline (token . span () , error) ; } else if token . kind () == TokenKind :: Comment { on_comment (tokens , token , receiver , error) ; } else { receiver . error (token . span () , error) ; } }
};
}
