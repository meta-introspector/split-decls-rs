// Generated macro for var_access_by_loop_index (function)
macro_rules! Depcrate_renderer_tests_square_bracketsvar_access_by_loop_index {
() => {
// Module: crate::renderer::tests::square_brackets
// Provides: {"var_access_by_loop_index"}
// Dependencies: {}
# [test] fn var_access_by_loop_index () { let context = Context :: new () ; let res = Tera :: one_off (r#"
{% set ics = ["fa-rocket","fa-paper-plane","fa-diamond","fa-signal"] %}
{% for a in ics %}
{{ ics[loop.index0] }}
{% endfor %}
    "# , & context , true ,) ; assert ! (res . is_ok ()) ; }
};
}
