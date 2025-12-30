// Generated macro for var_access_by_loop_index_with_set (function)
macro_rules! Depcrate_renderer_tests_square_bracketsvar_access_by_loop_index_with_set {
() => {
// Module: crate::renderer::tests::square_brackets
// Provides: {"var_access_by_loop_index_with_set"}
// Dependencies: {}
# [test] fn var_access_by_loop_index_with_set () { let context = Context :: new () ; let res = Tera :: one_off (r#"
{% set ics = ["fa-rocket","fa-paper-plane","fa-diamond","fa-signal"] %}
{% for a in ics %}
    {% set i = loop.index - 1 %}
    {{ ics[i] }}
{% endfor %}
    "# , & context , true ,) ; assert ! (res . is_ok ()) ; }
};
}
