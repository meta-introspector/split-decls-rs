// Generated macro for assert_parser_error (function)
macro_rules! Depcrate_test_parserassert_parser_error {
() => {
// Module: crate::test::parser
// Provides: {"assert_parser_error"}
// Dependencies: {}
fn assert_parser_error (filename : & str) { let file = PathBuf :: from (filename) ; let config = read_config (& file) ; let mut session = Session :: < io :: Stdout > :: new (config , None) ; let _ = session . format (Input :: File (filename . into ())) . unwrap () ; assert ! (session . has_parsing_errors ()) ; }
};
}
