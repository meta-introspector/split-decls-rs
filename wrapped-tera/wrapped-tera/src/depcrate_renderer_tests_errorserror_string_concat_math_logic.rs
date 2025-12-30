// Generated macro for error_string_concat_math_logic (function)
macro_rules! Depcrate_renderer_tests_errorserror_string_concat_math_logic {
() => {
// Module: crate::renderer::tests::errors
// Provides: {"error_string_concat_math_logic"}
// Dependencies: {}
# [test] fn error_string_concat_math_logic () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("tpl" , "{{ 'ho' ~ name < 10 }}")]) . unwrap () ; let mut context = Context :: new () ; context . insert ("name" , & "john") ; let result = tera . render ("tpl" , & context) ; assert_eq ! (result . unwrap_err () . source () . unwrap () . to_string () , "Tried to do math with a string concatenation: 'ho' ~ name") ; }
};
}
