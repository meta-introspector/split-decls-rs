// Generated macro for parse_include_tag (function)
macro_rules! Depcrate_parser_tests_parserparse_include_tag {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_include_tag"}
// Dependencies: {}
# [test] fn parse_include_tag () { let ast = parse ("{% include \"index.html\" -%}") . unwrap () ; assert_eq ! (ast [0] , Node :: Include (WS { left : false , right : true } , vec ! ["index.html" . to_string ()] , false ,) ,) ; let ast = parse ("{% include [\"custom/index.html\", \"index.html\"] ignore missing %}") . unwrap () ; assert_eq ! (ast [0] , Node :: Include (WS { left : false , right : false } , vec ! ["custom/index.html" . to_string () , "index.html" . to_string ()] , true ,) ,) ; }
};
}
