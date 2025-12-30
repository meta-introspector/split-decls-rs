// Generated macro for filter_args_are_not_escaped (function)
macro_rules! Depcrate_renderer_tests_basicfilter_args_are_not_escaped {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"filter_args_are_not_escaped"}
// Dependencies: {}
# [test] fn filter_args_are_not_escaped () { let mut context = Context :: new () ; context . insert ("my_var" , & "hey") ; context . insert ("to" , & "&") ; let input = r#"{{ my_var | replace(from="h", to=to) }}"# ; assert_eq ! (render_template (input , & context) . unwrap () , "&amp;ey") ; }
};
}
