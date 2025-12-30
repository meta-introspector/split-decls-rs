// Generated macro for parse_test (function)
macro_rules! Depcrate_parser_tests_parserparse_test {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_test"}
// Dependencies: {}
# [test] fn parse_test () { let ast = parse ("{{ a is divisibleby(2) }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Test (Test { ident : "a" . to_string () , negated : false , name : "divisibleby" . to_string () , args : vec ! [Expr :: new (ExprVal :: Int (2))] })))) ; }
};
}
