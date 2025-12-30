// Generated macro for parse_variable_tag_global_function_with_filter (function)
macro_rules! Depcrate_parser_tests_parserparse_variable_tag_global_function_with_filter {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_variable_tag_global_function_with_filter"}
// Dependencies: {}
# [test] fn parse_variable_tag_global_function_with_filter () { let ast = parse ("{{ get_time(some=1) | round | upper }}") . unwrap () ; let mut args = HashMap :: new () ; args . insert ("some" . to_string () , Expr :: new (ExprVal :: Int (1))) ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: with_filters (ExprVal :: FunctionCall (FunctionCall { name : "get_time" . to_string () , args } ,) , vec ! [FunctionCall { name : "round" . to_string () , args : HashMap :: new () } , FunctionCall { name : "upper" . to_string () , args : HashMap :: new () } ,] ,))) ; }
};
}
