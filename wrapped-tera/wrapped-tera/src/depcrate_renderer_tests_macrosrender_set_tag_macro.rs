// Generated macro for render_set_tag_macro (function)
macro_rules! Depcrate_renderer_tests_macrosrender_set_tag_macro {
() => {
// Module: crate::renderer::tests::macros
// Provides: {"render_set_tag_macro"}
// Dependencies: {}
# [test] fn render_set_tag_macro () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello()%}Hello{% endmacro hello %}") , ("hello.html" , "{% import \"macros\" as macros %}{% set my_var = macros::hello() %}{{my_var}}" ,) ,]) . unwrap () ; let result = tera . render ("hello.html" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "Hello" . to_string ()) ; }
};
}
