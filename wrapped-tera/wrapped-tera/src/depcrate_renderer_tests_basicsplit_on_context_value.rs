// Generated macro for split_on_context_value (function)
macro_rules! Depcrate_renderer_tests_basicsplit_on_context_value {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"split_on_context_value"}
// Dependencies: {}
# [test] fn split_on_context_value () { let mut tera = Tera :: default () ; tera . add_raw_template ("split.html" , r#"{{ body | split(pat="\n") }}"#) . unwrap () ; let mut context = Context :: new () ; context . insert ("body" , "multi\nple\nlines") ; let res = tera . render ("split.html" , & context) ; assert_eq ! (res . unwrap () , "[multi, ple, lines]") ; }
};
}
