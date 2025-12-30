// Generated macro for parse_variable_tag_simple_logic_expression (function)
macro_rules! Depcrate_parser_tests_parserparse_variable_tag_simple_logic_expression {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_variable_tag_simple_logic_expression"}
// Dependencies: {}
# [test] fn parse_variable_tag_simple_logic_expression () { let ast = parse ("{{ 1 > 2 }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Logic (LogicExpr { lhs : Box :: new (Expr :: new (ExprVal :: Int (1))) , operator : LogicOperator :: Gt , rhs : Box :: new (Expr :: new (ExprVal :: Int (2))) , } ,)))) ; }
};
}
