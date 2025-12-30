// Generated macro for parse_comments_before_extends (function)
macro_rules! Depcrate_parser_tests_parserparse_comments_before_extends {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_comments_before_extends"}
// Dependencies: {}
# [test] fn parse_comments_before_extends () { let ast = parse ("{# A comment #}{% extends \"index.html\" -%}") . unwrap () ; assert_eq ! (ast [0] , Node :: Extends (WS { left : false , right : true } , "index.html" . to_string () ,) ,) ; }
};
}
