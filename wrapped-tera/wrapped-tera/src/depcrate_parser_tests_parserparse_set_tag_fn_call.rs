// Generated macro for parse_set_tag_fn_call (function)
macro_rules! Depcrate_parser_tests_parserparse_set_tag_fn_call {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_set_tag_fn_call"}
// Dependencies: {}
# [test] fn parse_set_tag_fn_call () { let ast = parse ("{% set hello = utcnow() %}") . unwrap () ; assert_eq ! (ast [0] , Node :: Set (WS :: default () , Set { key : "hello" . to_string () , value : Expr :: new (ExprVal :: FunctionCall (FunctionCall { name : "utcnow" . to_string () , args : HashMap :: new () , } ,)) , global : false , } ,)) ; }
};
}
