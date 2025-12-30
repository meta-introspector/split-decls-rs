// Generated macro for lex_extends_with_imports (function)
macro_rules! Depcrate_parser_tests_lexerlex_extends_with_imports {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_extends_with_imports"}
// Dependencies: {}
# [test] fn lex_extends_with_imports () { let sample = r#"
{% extends "base.html" %}

{% import "macros/image.html" as image %}
{% import "macros/masonry.html" as masonry %}
{% import "macros/breadcrumb.html" as breadcrumb %}
{% import "macros/ul_links.html" as ul_links %}
{% import "macros/location.html" as location %}
         "# ; assert_lex_rule ! (Rule :: template , sample) ; }
};
}
