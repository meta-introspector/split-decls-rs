// Generated macro for render_macros_defined_in_template (function)
macro_rules! Depcrate_renderer_tests_macrosrender_macros_defined_in_template {
() => {
// Module: crate::renderer::tests::macros
// Provides: {"render_macros_defined_in_template"}
// Dependencies: {}
# [test] fn render_macros_defined_in_template () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("tpl" , "{% macro hello()%}Hello{% endmacro hello %}{% block hey %}{{self::hello()}}{% endblock hey %}") ,]) . unwrap () ; let result = tera . render ("tpl" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "Hello" . to_string ()) ; }
};
}
