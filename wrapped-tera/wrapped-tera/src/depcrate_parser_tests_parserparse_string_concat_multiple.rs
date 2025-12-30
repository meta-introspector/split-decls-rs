// Generated macro for parse_string_concat_multiple (function)
macro_rules! Depcrate_parser_tests_parserparse_string_concat_multiple {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_string_concat_multiple"}
// Dependencies: {}
# [test] fn parse_string_concat_multiple () { let ast = parse ("{{ `hello` ~ ident ~ 'ho' }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: StringConcat (StringConcat { values : vec ! [ExprVal :: String ("hello" . to_string ()) , ExprVal :: Ident ("ident" . to_string ()) , ExprVal :: String ("ho" . to_string ()) ,] }))) ,) ; }
};
}
