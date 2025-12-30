// Generated macro for error_unknown_index_variable (function)
macro_rules! Depcrate_renderer_tests_errorserror_unknown_index_variable {
() => {
// Module: crate::renderer::tests::errors
// Provides: {"error_unknown_index_variable"}
// Dependencies: {}
# [test] fn error_unknown_index_variable () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("tpl" , "{{ arr[a] }}")]) . unwrap () ; let mut context = Context :: new () ; context . insert ("arr" , & [1 , 2 , 3]) ; let result = tera . render ("tpl" , & context) ; assert_eq ! (result . unwrap_err () . source () . unwrap () . to_string () , "Variable arr[a] can not be evaluated because: Variable `a` not found in context while rendering \'tpl\'") ; }
};
}
