// Generated macro for render_simple_string (function)
macro_rules! Depcrate_renderer_tests_basicrender_simple_string {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"render_simple_string"}
// Dependencies: {}
# [test] fn render_simple_string () { let result = render_template ("<h1>Hello world</h1>" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "<h1>Hello world</h1>" . to_owned ()) ; }
};
}
