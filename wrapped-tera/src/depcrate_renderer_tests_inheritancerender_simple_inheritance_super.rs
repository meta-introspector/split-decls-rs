// Generated macro for render_simple_inheritance_super (function)
macro_rules! Depcrate_renderer_tests_inheritancerender_simple_inheritance_super {
() => {
// Module: crate::renderer::tests::inheritance
// Provides: {"render_simple_inheritance_super"}
// Dependencies: {}
# [test] fn render_simple_inheritance_super () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("top" , "{% block main %}TOP{% endblock main %}") , ("bottom" , "{% extends \"top\" %}{% block main %}{{ super() }}MAIN{% endblock %}") ,]) . unwrap () ; let result = tera . render ("bottom" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "TOPMAIN" . to_string ()) ; }
};
}
