// Generated macro for lex_ident (function)
macro_rules! Depcrate_parser_tests_lexerlex_ident {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_ident"}
// Dependencies: {}
# [test] fn lex_ident () { let inputs = vec ! ["hello" , "hello_" , "hello_1" , "HELLO" , "_1"] ; for i in inputs { assert_lex_rule ! (Rule :: ident , i) ; } assert ! (TeraParser :: parse (Rule :: ident , "909") . is_err ()) ; }
};
}
