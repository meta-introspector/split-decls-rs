// Generated macro for lex_macro_tag (function)
macro_rules! Depcrate_parser_tests_lexerlex_macro_tag {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_macro_tag"}
// Dependencies: {}
# [test] fn lex_macro_tag () { let inputs = vec ! ["{%- macro tag() %}" , "{% macro my_block(name) -%}" , "{% macro my_block(name=42) %}" , "{% macro foo ( bar=\"baz\", qux=42 ) %}" ,] ; for i in inputs { assert_lex_rule ! (Rule :: macro_tag , i) ; } }
};
}
