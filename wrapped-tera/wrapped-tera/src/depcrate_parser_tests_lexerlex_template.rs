// Generated macro for lex_template (function)
macro_rules! Depcrate_parser_tests_lexerlex_template {
() => {
// Module: crate::parser::tests::lexer
// Provides: {"lex_template"}
// Dependencies: {}
# [test] fn lex_template () { assert ! (TeraParser :: parse (Rule :: template , "{# Greeter template #}
            Hello {% if i18n %}世界{% else %}world{% endif %}
            {% for country in countries %}
                {{ loop.index }}.{{ country }}
            {% endfor %}" ,) . is_ok ()) ; }
};
}
