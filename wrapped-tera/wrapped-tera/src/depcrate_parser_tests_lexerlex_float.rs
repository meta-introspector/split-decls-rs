// Generated macro for lex_float (function)
macro_rules! Depcrate_parser_tests_lexerlex_float {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_float"}
// Dependencies: {}
# [test] fn lex_float () { let inputs = vec ! ["123.5" , "123.5" , "0.1" , "-1.1"] ; for i in inputs { assert_lex_rule ! (Rule :: float , i) ; } }
};
}
