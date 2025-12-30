// Generated macro for lex_int (function)
macro_rules! Depcrate_parser_tests_lexerlex_int {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_int"}
// Dependencies: {}
# [test] fn lex_int () { let inputs = vec ! ["-10" , "0" , "100" , "250000"] ; for i in inputs { assert_lex_rule ! (Rule :: int , i) ; } }
};
}
