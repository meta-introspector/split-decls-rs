// Generated macro for render_simple_inheritance (function)
macro_rules! Depcrate_renderer_tests_inheritancerender_simple_inheritance {
() => {
// Module: crate::renderer::tests::inheritance
// Provides: {"render_simple_inheritance"}
// Dependencies: {}
# [test] fn render_simple_inheritance () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("top" , "{% block pre %}{% endblock pre %}{% block main %}{% endblock main %}") , ("bottom" , "{% extends \"top\" %}{% block main %}MAIN{% endblock %}") ,]) . unwrap () ; let result = tera . render ("bottom" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "MAIN" . to_string ()) ; }
};
}
