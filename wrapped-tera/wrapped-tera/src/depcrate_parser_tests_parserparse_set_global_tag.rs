// Generated macro for parse_set_global_tag (function)
macro_rules! Depcrate_parser_tests_parserparse_set_global_tag {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_set_global_tag"}
// Dependencies: {}
# [test] fn parse_set_global_tag () { let ast = parse ("{% set_global hello = utcnow() %}") . unwrap () ; assert_eq ! (ast [0] , Node :: Set (WS :: default () , Set { key : "hello" . to_string () , value : Expr :: new (ExprVal :: FunctionCall (FunctionCall { name : "utcnow" . to_string () , args : HashMap :: new () , } ,)) , global : true , } ,)) ; }
};
}
