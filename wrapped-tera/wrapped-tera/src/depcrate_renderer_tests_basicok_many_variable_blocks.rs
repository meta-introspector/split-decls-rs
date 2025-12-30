// Generated macro for ok_many_variable_blocks (function)
macro_rules! Depcrate_renderer_tests_basicok_many_variable_blocks {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"ok_many_variable_blocks"}
// Dependencies: {}
# [test] fn ok_many_variable_blocks () { let mut context = Context :: new () ; context . insert ("username" , & "bob") ; let mut tpl = String :: new () ; for _ in 0 .. 200 { tpl . push_str ("{{ username }}") } let mut expected = String :: new () ; for _ in 0 .. 200 { expected . push_str ("bob") } assert_eq ! (render_template (& tpl , & context) . unwrap () , expected) ; }
};
}
