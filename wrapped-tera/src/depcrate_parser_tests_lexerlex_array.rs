// Generated macro for lex_array (function)
macro_rules! Depcrate_parser_tests_lexerlex_array {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_array"}
// Dependencies: {}
# [test] fn lex_array () { let inputs = vec ! ["[]" , "[1,2,3]" , "[1, 2,3,]" , "[1 + 1, 2,3 * 2,]" , "[\"foo\", \"bar\"]" , "[1,true,'string', 0.5, hello(), macros::hey(arg=1)]" ,] ; for i in inputs { assert_lex_rule ! (Rule :: array , i) ; } }
};
}
