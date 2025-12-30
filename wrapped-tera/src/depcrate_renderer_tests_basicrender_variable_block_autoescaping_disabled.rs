// Generated macro for render_variable_block_autoescaping_disabled (function)
macro_rules! Depcrate_renderer_tests_basicrender_variable_block_autoescaping_disabled {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"render_variable_block_autoescaping_disabled"}
// Dependencies: {}
# [test] fn render_variable_block_autoescaping_disabled () { let mut context = Context :: new () ; context . insert ("name" , & "john") ; context . insert ("malicious" , & "<html>") ; let inputs = vec ! [("{{ name }}" , "john") , ("{{ malicious }}" , "<html>") , ("{{ malicious | safe }}" , "<html>") , ("{{ malicious | upper }}" , "<HTML>") , ("{{ malicious | upper | safe }}" , "<HTML>") , ("{{ malicious | safe | upper }}" , "<HTML>") ,] ; for (input , expected) in inputs { let mut tera = Tera :: default () ; tera . add_raw_template ("hello.sql" , input) . unwrap () ; assert_eq ! (tera . render ("hello.sql" , & context) . unwrap () , expected) ; } }
};
}
