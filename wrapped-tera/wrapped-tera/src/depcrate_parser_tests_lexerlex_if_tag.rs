// Generated macro for lex_if_tag (function)
macro_rules! Depcrate_parser_tests_lexerlex_if_tag {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_if_tag"}
// Dependencies: {}
# [test] fn lex_if_tag () { let inputs = vec ! ["{%- if name %}" , "{% if true -%}" , "{% if admin or show %}" , "{% if 1 + 2 == 2 and true %}" , "{% if 1 + 2 == 2 and admin is defined %}" ,] ; for i in inputs { assert_lex_rule ! (Rule :: if_tag , i) ; } }
};
}
