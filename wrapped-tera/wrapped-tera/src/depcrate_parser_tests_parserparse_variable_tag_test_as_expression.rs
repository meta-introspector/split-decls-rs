// Generated macro for parse_variable_tag_test_as_expression (function)
macro_rules! Depcrate_parser_tests_parserparse_variable_tag_test_as_expression {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_variable_tag_test_as_expression"}
// Dependencies: {}
# [test] fn parse_variable_tag_test_as_expression () { let ast = parse ("{{ user is defined and user.admin }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Logic (LogicExpr { lhs : Box :: new (Expr :: new (ExprVal :: Test (Test { ident : "user" . to_string () , negated : false , name : "defined" . to_string () , args : vec ! [] , } ,))) , operator : LogicOperator :: And , rhs : Box :: new (Expr :: new (ExprVal :: Ident ("user.admin" . to_string ()))) , } ,)))) ; }
};
}
