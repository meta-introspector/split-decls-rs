// Generated macro for error_location_in_parent_block (function)
macro_rules! Depcrate_renderer_tests_errorserror_location_in_parent_block {
() => {
// Module: crate::renderer::tests::errors
// Provides: {"error_location_in_parent_block"}
// Dependencies: {}
# [test] fn error_location_in_parent_block () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("parent" , "Hello {{ greeting }} {% block bob %}{{ 1 + true }}{% endblock bob %}") , ("child" , "{% extends \"parent\" %}{% block bob %}{{ super() }}Hey{% endblock bob %}") ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap_err () . to_string () , "Failed to render \'child\' (error happened in 'parent').") ; }
};
}
