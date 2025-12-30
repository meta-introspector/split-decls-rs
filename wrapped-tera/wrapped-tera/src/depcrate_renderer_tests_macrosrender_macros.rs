// Generated macro for render_macros (function)
macro_rules! Depcrate_renderer_tests_macrosrender_macros {
() => {
// Module: crate::renderer::tests::macros
// Provides: {"render_macros"}
// Dependencies: {}
# [test] fn render_macros () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello()%}Hello{% endmacro hello %}") , ("tpl" , "{% import \"macros\" as macros %}{% block hey %}{{macros::hello()}}{% endblock hey %}" ,) ,]) . unwrap () ; let result = tera . render ("tpl" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "Hello" . to_string ()) ; }
};
}
