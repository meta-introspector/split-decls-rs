// Generated macro for parse_variable_tag_negated_expr (function)
macro_rules! Depcrate_parser_tests_parserparse_variable_tag_negated_expr {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_variable_tag_negated_expr"}
// Dependencies: {}
# [test] fn parse_variable_tag_negated_expr () { let ast = parse ("{{ not id and not true and not 1 + 1 }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Logic (LogicExpr { lhs : Box :: new (Expr :: new (ExprVal :: Logic (LogicExpr { lhs : Box :: new (Expr :: new_negated (ExprVal :: Ident ("id" . to_string ()))) , operator : LogicOperator :: And , rhs : Box :: new (Expr :: new_negated (ExprVal :: Bool (true))) , } ,))) , operator : LogicOperator :: And , rhs : Box :: new (Expr :: new_negated (ExprVal :: Math (MathExpr { lhs : Box :: new (Expr :: new (ExprVal :: Int (1))) , operator : MathOperator :: Add , rhs : Box :: new (Expr :: new (ExprVal :: Int (1))) , } ,))) , } ,)))) ; }
};
}
