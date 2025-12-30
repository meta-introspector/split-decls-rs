// Generated macro for parse_variable_tag_global_function (function)
macro_rules! Depcrate_parser_tests_parserparse_variable_tag_global_function {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_variable_tag_global_function"}
// Dependencies: {}
# [test] fn parse_variable_tag_global_function () { let ast = parse ("{{ get_time(some=1) }}") . unwrap () ; let mut args = HashMap :: new () ; args . insert ("some" . to_string () , Expr :: new (ExprVal :: Int (1))) ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: FunctionCall (FunctionCall { name : "get_time" . to_string () , args } ,)))) ; }
};
}
