// Generated macro for parse_variable_tag_ident (function)
macro_rules! Depcrate_parser_tests_parserparse_variable_tag_ident {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_variable_tag_ident"}
// Dependencies: {}
# [test] fn parse_variable_tag_ident () { let ast = parse ("{{ id }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Ident ("id" . to_string ()))) ,) ; }
};
}
