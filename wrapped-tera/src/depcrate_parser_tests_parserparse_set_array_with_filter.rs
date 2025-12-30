// Generated macro for parse_set_array_with_filter (function)
macro_rules! Depcrate_parser_tests_parserparse_set_array_with_filter {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_set_array_with_filter"}
// Dependencies: {}
# [test] fn parse_set_array_with_filter () { let ast = parse ("{% set hello = [1, true, 'hello'] | length %}") . unwrap () ; assert_eq ! (ast [0] , Node :: Set (WS :: default () , Set { key : "hello" . to_string () , value : Expr :: with_filters (ExprVal :: Array (vec ! [Expr :: new (ExprVal :: Int (1)) , Expr :: new (ExprVal :: Bool (true)) , Expr :: new (ExprVal :: String ("hello" . to_string ())) ,]) , vec ! [FunctionCall { name : "length" . to_string () , args : HashMap :: new () } ,] ,) , global : false , } ,)) ; }
};
}
