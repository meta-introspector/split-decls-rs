// Generated macro for lex_filter (function)
macro_rules! Depcrate_parser_tests_lexerlex_filter {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_filter"}
// Dependencies: {}
# [test] fn lex_filter () { let inputs = vec ! ["|attr" , "|attr()" , "|attr(key=1)" , "|attr(key=1, more=true)" , "|attr(key=1,more=true)" ,] ; for i in inputs { assert_lex_rule ! (Rule :: filter , i) ; } }
};
}
