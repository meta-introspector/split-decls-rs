// Generated macro for lex_test (function)
macro_rules! Depcrate_parser_tests_lexerlex_test {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_test"}
// Dependencies: {}
# [test] fn lex_test () { let inputs = vec ! ["a is defined" , "a is defined()" , "a is divisibleby(2)" , "a is in([1, 2, something])"] ; for i in inputs { assert ! (TeraParser :: parse (Rule :: test , i) . is_ok ()) ; } }
};
}
