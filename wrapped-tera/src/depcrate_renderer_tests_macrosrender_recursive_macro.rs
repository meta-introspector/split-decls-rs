// Generated macro for render_recursive_macro (function)
macro_rules! Depcrate_renderer_tests_macrosrender_recursive_macro {
() => {
// Module: crate::renderer::tests::macros
// Provides: {"render_recursive_macro"}
// Dependencies: {}
# [test] fn render_recursive_macro () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro factorial(n) %}{% if n > 1 %}{{ n }} - {{ self::factorial(n=n-1) }}{% else %}1{% endif %}{{ n }}{% endmacro factorial %}" ,) , ("hello.html" , "{% import \"macros\" as macros %}{{macros::factorial(n=7)}}") ,]) . unwrap () ; let result = tera . render ("hello.html" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "7 - 6 - 5 - 4 - 3 - 2 - 11234567" . to_string ()) ; }
};
}
