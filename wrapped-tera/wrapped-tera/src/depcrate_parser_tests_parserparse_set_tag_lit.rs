// Generated macro for parse_set_tag_lit (function)
macro_rules! Depcrate_parser_tests_parserparse_set_tag_lit {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_set_tag_lit"}
// Dependencies: {}
# [test] fn parse_set_tag_lit () { let ast = parse ("{% set hello = \"hi\" %}") . unwrap () ; assert_eq ! (ast [0] , Node :: Set (WS :: default () , Set { key : "hello" . to_string () , value : Expr :: new (ExprVal :: String ("hi" . to_string ())) , global : false , } ,)) ; }
};
}
