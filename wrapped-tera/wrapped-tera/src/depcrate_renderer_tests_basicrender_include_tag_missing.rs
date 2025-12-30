// Generated macro for render_include_tag_missing (function)
macro_rules! Depcrate_renderer_tests_basicrender_include_tag_missing {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"render_include_tag_missing"}
// Dependencies: {}
# [test] fn render_include_tag_missing () { let mut tera = Tera :: default () ; tera . add_raw_template ("hello" , "<h1>Hello {% include \"world\" %}</h1>") . unwrap () ; let result = tera . render ("hello" , & Context :: new ()) ; assert ! (result . is_err ()) ; let mut tera = Tera :: default () ; tera . add_raw_template ("hello" , "<h1>Hello {% include \"world\" ignore missing %}</h1>") . unwrap () ; let result = tera . render ("hello" , & Context :: new ()) . unwrap () ; assert_eq ! (result , "<h1>Hello </h1>" . to_owned ()) ; }
};
}
