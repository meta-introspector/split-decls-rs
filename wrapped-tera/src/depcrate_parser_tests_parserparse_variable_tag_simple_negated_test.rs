// Generated macro for parse_variable_tag_simple_negated_test (function)
macro_rules! Depcrate_parser_tests_parserparse_variable_tag_simple_negated_test {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_variable_tag_simple_negated_test"}
// Dependencies: {}
# [test] fn parse_variable_tag_simple_negated_test () { let ast = parse ("{{ id is not defined }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS :: default () , Expr :: new (ExprVal :: Test (Test { ident : "id" . to_string () , negated : true , name : "defined" . to_string () , args : vec ! [] , } ,)))) ; }
};
}
