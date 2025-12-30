// Generated macro for lex_content (function)
macro_rules! Depcrate_parser_tests_lexerlex_content {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_content"}
// Dependencies: {}
# [test] fn lex_content () { let inputs = vec ! ["some text" , "{{ name }}" , "{# comment #}" , "{% filter upper %}hey{% endfilter %}" , "{% filter upper() %}hey{% endfilter %}" , "{% raw %}{{ hey }}{% endraw %}" , "{% for a in b %}{{a}}{% endfor %}" , "{% if i18n %}世界{% else %}world{% endif %}" ,] ; for i in inputs { assert_lex_rule ! (Rule :: content , i) ; } }
};
}
