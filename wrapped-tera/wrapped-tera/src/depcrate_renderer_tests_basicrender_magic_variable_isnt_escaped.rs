// Generated macro for render_magic_variable_isnt_escaped (function)
macro_rules! Depcrate_renderer_tests_basicrender_magic_variable_isnt_escaped {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"render_magic_variable_isnt_escaped"}
// Dependencies: {}
# [test] fn render_magic_variable_isnt_escaped () { let mut context = Context :: new () ; context . insert ("html" , & "<html>") ; let result = render_template ("{{ __tera_context }}" , & context) ; assert_eq ! (result . unwrap () , r#"{
  "html": "<html>"
}"# . to_owned ()) ; }
};
}
