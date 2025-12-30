// Generated macro for parse_string_concat_can_merge (function)
macro_rules! Depcrate_parser_tests_parserparse_string_concat_can_merge {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_string_concat_can_merge"}
// Dependencies: {}
# [test] fn parse_string_concat_can_merge () { let ast = parse ("{{ `hello` ~ 'hey' }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: String ("hellohey" . to_string ()))) ,) ; }
};
}
