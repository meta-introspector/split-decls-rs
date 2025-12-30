// Generated macro for parse_raw_tag_with_ws (function)
macro_rules! Depcrate_parser_tests_parserparse_raw_tag_with_ws {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_raw_tag_with_ws"}
// Dependencies: {}
# [test] fn parse_raw_tag_with_ws () { let ast = parse ("{% raw %}    yaml_test:     {% endraw %}") . unwrap () ; let start_ws = WS :: default () ; let end_ws = WS :: default () ; assert_eq ! (ast [0] , Node :: Raw (start_ws , "    yaml_test:     " . to_string () , end_ws)) ; }
};
}
