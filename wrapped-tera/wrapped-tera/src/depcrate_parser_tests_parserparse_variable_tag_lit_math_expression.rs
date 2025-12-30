// Generated macro for parse_variable_tag_lit_math_expression (function)
macro_rules! Depcrate_parser_tests_parserparse_variable_tag_lit_math_expression {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_variable_tag_lit_math_expression"}
// Dependencies: {}
# [test] fn parse_variable_tag_lit_math_expression () { let ast = parse ("{{ count + 1 * 2.5 }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Math (MathExpr { lhs : Box :: new (Expr :: new (ExprVal :: Ident ("count" . to_string ()))) , operator : MathOperator :: Add , rhs : Box :: new (Expr :: new (ExprVal :: Math (MathExpr { lhs : Box :: new (Expr :: new (ExprVal :: Int (1))) , operator : MathOperator :: Mul , rhs : Box :: new (Expr :: new (ExprVal :: Float (2.5))) , } ,))) , } ,))) ,) ; }
};
}
