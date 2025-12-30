// Generated macro for parse_text (function)
macro_rules! Depcrate_parser_tests_parserparse_text {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_text"}
// Dependencies: {}
# [test] fn parse_text () { let ast = parse ("hello world") . unwrap () ; assert_eq ! (ast [0] , Node :: Text ("hello world" . to_string ())) ; }
};
}
