// Generated macro for parse_document (function)
macro_rules! Depcrate_parser_documentparse_document {
() => {
// Module: crate::parser::document
// Provides: {"parse_document"}
// Dependencies: {}
# [doc = " Parse lexed tokens into [`Event`][super::Event]s"] pub fn parse_document (tokens : & [Token] , receiver : & mut dyn EventReceiver , error : & mut dyn ErrorSink ,) { let mut tokens = TokenSlice :: new (tokens) ; # [cfg (feature = "debug")] let mut receiver = DebugEventReceiver :: new (receiver) ; # [cfg (feature = "debug")] let receiver = & mut receiver ; # [cfg (feature = "debug")] let mut error = DebugErrorSink :: new (error) ; # [cfg (feature = "debug")] let error = & mut error ; document (& mut tokens , receiver , error) ; eof (& mut tokens , receiver , error) ; }
};
}
