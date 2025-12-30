// Generated macro for parse_set_tag_macro_call (function)
macro_rules! Depcrate_parser_tests_parserparse_set_tag_macro_call {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_set_tag_macro_call"}
// Dependencies: {}
# [test] fn parse_set_tag_macro_call () { let ast = parse ("{% set hello = macros::something() %}") . unwrap () ; assert_eq ! (ast [0] , Node :: Set (WS :: default () , Set { key : "hello" . to_string () , value : Expr :: new (ExprVal :: MacroCall (MacroCall { namespace : "macros" . to_string () , name : "something" . to_string () , args : HashMap :: new () , } ,)) , global : false , } ,)) ; }
};
}
