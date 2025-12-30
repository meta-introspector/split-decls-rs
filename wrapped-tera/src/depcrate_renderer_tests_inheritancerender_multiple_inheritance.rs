// Generated macro for render_multiple_inheritance (function)
macro_rules! Depcrate_renderer_tests_inheritancerender_multiple_inheritance {
() => {
// Module: crate::renderer::tests::inheritance
// Provides: {"render_multiple_inheritance"}
// Dependencies: {}
# [test] fn render_multiple_inheritance () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("top" , "{% block pre %}{% endblock pre %}{% block main %}{% endblock main %}") , ("mid" , "{% extends \"top\" %}{% block pre %}PRE{% endblock pre %}") , ("bottom" , "{% extends \"mid\" %}{% block main %}MAIN{% endblock main %}") ,]) . unwrap () ; let result = tera . render ("bottom" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "PREMAIN" . to_string ()) ; }
};
}
