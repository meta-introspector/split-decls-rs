// Generated macro for error_location_base_template (function)
macro_rules! Depcrate_renderer_tests_errorserror_location_base_template {
() => {
// Module: crate::renderer::tests::errors
// Provides: {"error_location_base_template"}
// Dependencies: {}
# [test] fn error_location_base_template () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("parent" , "Hello {{ greeting + 1}} {% block bob %}{% endblock bob %}") , ("child" , "{% extends \"parent\" %}{% block bob %}Hey{% endblock bob %}") ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap_err () . to_string () , "Failed to render \'child\' (error happened in 'parent').") ; }
};
}
