// Generated macro for lex_string (function)
macro_rules! Depcrate_parser_tests_lexerlex_string {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_string"}
// Dependencies: {}
# [test] fn lex_string () { let inputs = vec ! ["\"Blabla\"" , "\"123\"" , "\'123\'" , "\'This is still a string\'" , "`this is backquted`" , "`and this too`" ,] ; for i in inputs { assert_lex_rule ! (Rule :: string , i) ; } }
};
}
