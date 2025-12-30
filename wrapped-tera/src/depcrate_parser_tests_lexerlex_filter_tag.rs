// Generated macro for lex_filter_tag (function)
macro_rules! Depcrate_parser_tests_lexerlex_filter_tag {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_filter_tag"}
// Dependencies: {}
# [test] fn lex_filter_tag () { let inputs = vec ! ["{%- filter tag() %}" , "{% filter foo(bar=baz) -%}" , "{% filter foo(bar=42) %}" , "{% filter foo(bar=baz,qux=quz) %}" , "{% filter foo(bar=baz, qux=quz) %}" , "{% filter foo ( bar=\"baz\", qux=42 ) %}" ,] ; for i in inputs { assert_lex_rule ! (Rule :: filter_tag , i) ; } }
};
}
