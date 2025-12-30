// Generated macro for stdin_parser_panic_caught (function)
macro_rules! Depcrate_teststdin_parser_panic_caught {
() => {
// Module: crate::test
// Provides: {"stdin_parser_panic_caught"}
// Dependencies: {}
# [test] fn stdin_parser_panic_caught () { init_log () ; for text in ["{" , "}"] . iter () . cloned () . map (String :: from) { let mut buf = vec ! [] ; let mut session = Session :: new (Default :: default () , Some (& mut buf)) ; let _ = session . format (Input :: Text (text)) ; assert ! (session . has_parsing_errors ()) ; } }
};
}
