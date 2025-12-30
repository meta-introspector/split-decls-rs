// Generated macro for error_out_of_range_index (function)
macro_rules! Depcrate_renderer_tests_errorserror_out_of_range_index {
() => {
// Module: crate::renderer::tests::errors
// Provides: {"error_out_of_range_index"}
// Dependencies: {}
# [test] fn error_out_of_range_index () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("tpl" , "{{ arr[10] }}")]) . unwrap () ; let mut context = Context :: new () ; context . insert ("arr" , & [1 , 2 , 3]) ; let result = tera . render ("tpl" , & Context :: new ()) ; assert_eq ! (result . unwrap_err () . source () . unwrap () . to_string () , "Variable `arr[10]` not found in context while rendering \'tpl\': the evaluated version was `arr.10`. Maybe the index is out of bounds?") ; }
};
}
