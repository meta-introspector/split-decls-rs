// Generated macro for parse_string_concat (function)
macro_rules! Depcrate_parser_tests_parserparse_string_concat {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_string_concat"}
// Dependencies: {}
# [test] fn parse_string_concat () { let ast = parse ("{{ `hello` ~ ident }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: StringConcat (StringConcat { values : vec ! [ExprVal :: String ("hello" . to_string ()) , ExprVal :: Ident ("ident" . to_string ()) ,] }))) ,) ; }
};
}
