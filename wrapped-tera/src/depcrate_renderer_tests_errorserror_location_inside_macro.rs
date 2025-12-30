// Generated macro for error_location_inside_macro (function)
macro_rules! Depcrate_renderer_tests_errorserror_location_inside_macro {
() => {
// Module: crate::renderer::tests::errors
// Provides: {"error_location_inside_macro"}
// Dependencies: {}
# [test] fn error_location_inside_macro () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello()%}{{ 1 + true }}{% endmacro hello %}") , ("tpl" , "{% import \"macros\" as macros %}{{ macros::hello() }}") ,]) . unwrap () ; let result = tera . render ("tpl" , & Context :: new ()) ; assert_eq ! (result . unwrap_err () . to_string () , "Failed to render \'tpl\': error while rendering macro `macros::hello`") ; }
};
}
