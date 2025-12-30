// Generated macro for render_macros_with_default_args (function)
macro_rules! Depcrate_renderer_tests_macrosrender_macros_with_default_args {
() => {
// Module: crate::renderer::tests::macros
// Provides: {"render_macros_with_default_args"}
// Dependencies: {}
# [test] fn render_macros_with_default_args () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello(val=1) %}{{val}}{% endmacro hello %}") , ("hello.html" , "{% import \"macros\" as macros %}{{macros::hello()}}") ,]) . unwrap () ; let result = tera . render ("hello.html" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "1" . to_string ()) ; }
};
}
