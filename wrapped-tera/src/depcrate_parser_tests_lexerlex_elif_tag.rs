// Generated macro for lex_elif_tag (function)
macro_rules! Depcrate_parser_tests_lexerlex_elif_tag {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_elif_tag"}
// Dependencies: {}
# [test] fn lex_elif_tag () { let inputs = vec ! ["{%- elif name %}" , "{% elif true -%}" , "{% elif admin or show %}" , "{% elif 1 + 2 == 2 and true %}" , "{% elif 1 + 2 == 2 and admin is defined %}" ,] ; for i in inputs { assert_lex_rule ! (Rule :: elif_tag , i) ; } }
};
}
