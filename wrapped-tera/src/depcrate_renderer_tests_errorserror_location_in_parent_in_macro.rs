// Generated macro for error_location_in_parent_in_macro (function)
macro_rules! Depcrate_renderer_tests_errorserror_location_in_parent_in_macro {
() => {
// Module: crate::renderer::tests::errors
// Provides: {"error_location_in_parent_in_macro"}
// Dependencies: {}
# [test] fn error_location_in_parent_in_macro () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello()%}{{ 1 + true }}{% endmacro hello %}") , ("parent" , "{% import \"macros\" as macros %}{{ macros::hello() }}{% block bob %}{% endblock bob %}") , ("child" , "{% extends \"parent\" %}{% block bob %}{{ super() }}Hey{% endblock bob %}") ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap_err () . to_string () , "Failed to render \'child\': error while rendering macro `macros::hello` (error happened in \'parent\').") ; }
};
}
