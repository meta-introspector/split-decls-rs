// Generated macro for parse_filter_section_without_args (function)
macro_rules! Depcrate_parser_tests_parserparse_filter_section_without_args {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_filter_section_without_args"}
// Dependencies: {}
# [test] fn parse_filter_section_without_args () { let ast = parse ("{% filter upper -%}A{%- endfilter %}") . unwrap () ; let start_ws = WS { right : true , .. Default :: default () } ; let end_ws = WS { left : true , .. Default :: default () } ; assert_eq ! (ast [0] , Node :: FilterSection (start_ws , FilterSection { filter : FunctionCall { name : "upper" . to_string () , args : HashMap :: new () } , body : vec ! [Node :: Text ("A" . to_string ())] , } , end_ws ,)) ; }
};
}
