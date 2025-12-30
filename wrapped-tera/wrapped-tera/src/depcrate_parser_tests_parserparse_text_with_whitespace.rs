// Generated macro for parse_text_with_whitespace (function)
macro_rules! Depcrate_parser_tests_parserparse_text_with_whitespace {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_text_with_whitespace"}
// Dependencies: {}
# [test] fn parse_text_with_whitespace () { let ast = parse (" hello world ") . unwrap () ; assert_eq ! (ast [0] , Node :: Text (" hello world " . to_string ())) ; }
};
}
