// Generated macro for parse_variable_with_whitespace_trimming (function)
macro_rules! Depcrate_parser_tests_parserparse_variable_with_whitespace_trimming {
() => {
// Module: crate::parser::tests::parser
// Provides: {"parse_variable_with_whitespace_trimming"}
// Dependencies: {}
# [test] fn parse_variable_with_whitespace_trimming () { let ast = parse ("{{- id }}") . unwrap () ; assert_eq ! (ast [0] , Node :: VariableBlock (WS { left : true , right : false } , Expr :: new (ExprVal :: Ident ("id" . to_string ()))) ,) ; }
};
}
