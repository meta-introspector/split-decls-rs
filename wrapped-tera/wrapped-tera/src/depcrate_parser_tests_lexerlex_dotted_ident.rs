// Generated macro for lex_dotted_ident (function)
macro_rules! Depcrate_parser_tests_lexerlex_dotted_ident {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_dotted_ident"}
// Dependencies: {}
# [test] fn lex_dotted_ident () { let inputs = vec ! ["hello" , "hello_" , "hello_1" , "HELLO" , "_1" , "hey.ho" , "h" , "ho" , "hey.ho.hu" , "hey.0" , "h.u" ,] ; for i in inputs { assert_lex_rule ! (Rule :: dotted_ident , i) ; } let invalid_inputs = vec ! ["." , "9.w"] ; for i in invalid_inputs { assert ! (TeraParser :: parse (Rule :: dotted_ident , i) . is_err ()) ; } }
};
}
