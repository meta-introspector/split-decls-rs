// Generated macro for lex_kwarg (function)
macro_rules! Depcrate_parser_tests_lexerlex_kwarg {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_kwarg"}
// Dependencies: {}
# [test] fn lex_kwarg () { let inputs = vec ! ["hello=1" , "hello=1+1" , "hello=[]" , "hello=[true, false]" , "hello1=true" , "hello=name" , "hello=name|filter" , "hello=name|filter(with_arg=true)" ,] ; for i in inputs { assert_lex_rule ! (Rule :: kwarg , i) ; } }
};
}
