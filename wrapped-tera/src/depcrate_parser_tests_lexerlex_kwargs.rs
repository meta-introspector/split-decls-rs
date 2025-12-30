// Generated macro for lex_kwargs (function)
macro_rules! Depcrate_parser_tests_lexerlex_kwargs {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_kwargs"}
// Dependencies: {}
# [test] fn lex_kwargs () { let inputs = vec ! ["hello=1" , "hello=1+1,hey=1" , "hello1=true,name=name,admin=true" , "hello=name" , "hello=name|filter,id=1" , "hello=name|filter(with_arg=true),id=1" ,] ; for i in inputs { assert_lex_rule ! (Rule :: kwargs , i) ; } }
};
}
