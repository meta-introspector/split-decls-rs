// Generated macro for lex_string_concat (function)
macro_rules! Depcrate_parser_tests_lexerlex_string_concat {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_string_concat"}
// Dependencies: {}
# [test] fn lex_string_concat () { let inputs = vec ! ["'hello' ~ `hey`" , "'hello' ~ 1" , "'hello' ~ 3.18" , "1 ~ 'hello'" , "3.18 ~ 'hello'" , "'hello' ~ ident" , "ident ~ 'hello'" , "'hello' ~ ident[0]" , "'hello' ~ a_function()" , "a_function() ~ 'hello'" , r#"'hello' ~ "hey""# , r#"a_string ~ " world""# , "'hello' ~ ident ~ `ho`" ,] ; for i in inputs { assert_lex_rule ! (Rule :: string_concat , i) ; } }
};
}
