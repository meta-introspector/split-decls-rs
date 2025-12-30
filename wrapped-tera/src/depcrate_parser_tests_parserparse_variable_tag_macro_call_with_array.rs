// Generated macro for parse_variable_tag_macro_call_with_array (function)
macro_rules! Depcrate_parser_tests_parserparse_variable_tag_macro_call_with_array {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_variable_tag_macro_call_with_array"}
// Dependencies: {}
# [test] fn parse_variable_tag_macro_call_with_array () { let ast = parse ("{{ macros::get_time(some=[1, 2]) }}") . unwrap () ; let mut args = HashMap :: new () ; args . insert ("some" . to_string () , Expr :: new (ExprVal :: Array (vec ! [Expr :: new (ExprVal :: Int (1)) , Expr :: new (ExprVal :: Int (2))])) ,) ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: MacroCall (MacroCall { namespace : "macros" . to_string () , name : "get_time" . to_string () , args , } ,)))) ; }
};
}
