// Generated macro for lex_macro_definition (function)
macro_rules! Depcrate_parser_tests_lexerlex_macro_definition {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_macro_definition"}
// Dependencies: {}
# [test] fn lex_macro_definition () { let inputs = vec ! ["hello()" , "hello(name, admin)" , "hello(name, admin=1)" , "hello(name=\"bob\", admin)" , "hello(name=\"bob\",admin=true)" ,] ; for i in inputs { assert ! (TeraParser :: parse (Rule :: macro_fn , i) . is_ok ()) ; } }
};
}
