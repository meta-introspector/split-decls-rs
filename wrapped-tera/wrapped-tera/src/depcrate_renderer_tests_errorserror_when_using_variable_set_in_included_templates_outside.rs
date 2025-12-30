// Generated macro for error_when_using_variable_set_in_included_templates_outside (function)
macro_rules! Depcrate_renderer_tests_errorserror_when_using_variable_set_in_included_templates_outside {
() => {
// Module: crate::renderer::tests::errors
// Provides: {"error_when_using_variable_set_in_included_templates_outside"}
// Dependencies: {}
# [test] fn error_when_using_variable_set_in_included_templates_outside () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("included" , r#"{{a}}{% set b = "hi" %}-{{b}}"#) , ("base" , r#"{{a}}{% include "included" %}{{b}}"#) ,]) . unwrap () ; let mut context = Context :: new () ; context . insert ("a" , & 10) ; let result = tera . render ("base" , & context) ; assert_eq ! (result . unwrap_err () . source () . unwrap () . to_string () , "Variable `b` not found in context while rendering \'base\'") ; }
};
}
