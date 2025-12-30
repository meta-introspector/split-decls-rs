// Generated macro for parse_raw_tag (function)
macro_rules! Depcrate_parser_tests_parserparse_raw_tag {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_raw_tag"}
// Dependencies: {}
# [test] fn parse_raw_tag () { let ast = parse ("{% raw -%}{{hey}}{%- endraw %}") . unwrap () ; let start_ws = WS { right : true , .. Default :: default () } ; let end_ws = WS { left : true , .. Default :: default () } ; assert_eq ! (ast [0] , Node :: Raw (start_ws , "{{hey}}" . to_string () , end_ws)) ; }
};
}
