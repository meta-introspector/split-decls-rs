// Generated macro for lex_set_tag (function)
macro_rules! Depcrate_parser_tests_lexerlex_set_tag {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_set_tag"}
// Dependencies: {}
# [test] fn lex_set_tag () { let inputs = vec ! ["{%- set a = true %}" , "{% set a = object -%}" , "{% set a = [1,2,3, 'hey'] -%}" , "{% set a = fn_call() %}" , "{% set a = fn_call(with_args=true, name=name) %}" , "{% set a = macros::fn_call(with_args=true, name=name) %}" , "{% set a = var | caps %}" , "{% set a = var +1 >= 2%}" ,] ; for i in inputs { assert_lex_rule ! (Rule :: set_tag , i) ; } }
};
}
