// Generated macro for parse_variable_math_on_filter (function)
macro_rules! Depcrate_parser_tests_parserparse_variable_math_on_filter {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_variable_math_on_filter"}
// Dependencies: {}
# [test] fn parse_variable_math_on_filter () { let ast = parse ("{{ a | length - 1 }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Math (MathExpr { lhs : Box :: new (Expr :: with_filters (ExprVal :: Ident ("a" . to_string ()) , vec ! [FunctionCall { name : "length" . to_string () , args : HashMap :: new () } ,] ,)) , operator : MathOperator :: Sub , rhs : Box :: new (Expr :: new (ExprVal :: Int (1))) , } ,)))) ; }
};
}
