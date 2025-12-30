// Generated macro for lex_fn_call (function)
macro_rules! Depcrate_parser_tests_lexerlex_fn_call {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_fn_call"}
// Dependencies: {}
# [test] fn lex_fn_call () { let inputs = vec ! ["fn(hello=1)" , "fn(hello=1+1,hey=1)" , "fn(hello1=true,name=name,admin=true)" , "fn(hello=name)" , "fn(hello=name,)" , "fn(\n  hello=name,\n)" , "fn(hello=name|filter,id=1)" ,] ; for i in inputs { assert_lex_rule ! (Rule :: fn_call , i) ; } }
};
}
