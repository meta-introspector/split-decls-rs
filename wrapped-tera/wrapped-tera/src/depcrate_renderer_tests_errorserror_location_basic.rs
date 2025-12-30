// Generated macro for error_location_basic (function)
macro_rules! Depcrate_renderer_tests_errorserror_location_basic {
() => {
// Module: crate::renderer::tests::errors
// Provides: {"error_location_basic"}
// Dependencies: {}
# [test] fn error_location_basic () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("tpl" , "{{ 1 + true }}")]) . unwrap () ; let result = tera . render ("tpl" , & Context :: new ()) ; assert_eq ! (result . unwrap_err () . to_string () , "Failed to render \'tpl\'") ; }
};
}
