// Generated macro for parse_variable_tag_simple_negated_expr (function)
macro_rules! Depcrate_parser_tests_parserparse_variable_tag_simple_negated_expr {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_variable_tag_simple_negated_expr"}
// Dependencies: {}
# [test] fn parse_variable_tag_simple_negated_expr () { let ast = parse ("{{ not id }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new_negated (ExprVal :: Ident ("id" . to_string ())))) ; }
};
}
