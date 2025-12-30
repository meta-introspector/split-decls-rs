// Generated macro for parse_extends (function)
macro_rules! Depcrate_parser_tests_parserparse_extends {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_extends"}
// Dependencies: {}
# [test] fn parse_extends () { let ast = parse ("{% extends \"index.html\" -%}") . unwrap () ; assert_eq ! (ast [0] , Node :: Extends (WS { left : false , right : true } , "index.html" . to_string () ,) ,) ; }
};
}
