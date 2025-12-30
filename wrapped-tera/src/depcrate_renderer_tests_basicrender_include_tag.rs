// Generated macro for render_include_tag (function)
macro_rules! Depcrate_renderer_tests_basicrender_include_tag {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"render_include_tag"}
// Dependencies: {}
# [test] fn render_include_tag () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("world" , "world") , ("hello" , "<h1>Hello {% include \"world\" %}</h1>") ,]) . unwrap () ; let result = tera . render ("hello" , & Context :: new ()) . unwrap () ; assert_eq ! (result , "<h1>Hello world</h1>" . to_owned ()) ; }
};
}
