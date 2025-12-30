// Generated macro for can_fail_rendering_from_template (function)
macro_rules! Depcrate_renderer_tests_basiccan_fail_rendering_from_template {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"can_fail_rendering_from_template"}
// Dependencies: {}
# [test] fn can_fail_rendering_from_template () { let mut context = Context :: new () ; context . insert ("title" , "hello") ; let res = render_template (r#"{{ throw(message="Error: " ~ title ~ " did not include a summary") }}"# , & context ,) ; let err = res . expect_err ("This should always fail to render") ; let source = err . source () . expect ("Must have a source") ; assert_eq ! (source . to_string () , "Function call 'throw' failed") ; let source = source . source () . expect ("Should have a nested error") ; assert_eq ! (source . to_string () , "Error: hello did not include a summary") ; }
};
}
