// Generated macro for lex_boolean (function)
macro_rules! Depcrate_parser_tests_lexerlex_boolean {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_boolean"}
// Dependencies: {}
# [test] fn lex_boolean () { let inputs = vec ! ["true" , "false" , "True" , "False"] ; for i in inputs { assert_lex_rule ! (Rule :: boolean , i) ; } }
};
}
