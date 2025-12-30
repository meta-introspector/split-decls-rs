// Generated macro for render_filter_section_inheritance (function)
macro_rules! Depcrate_renderer_tests_inheritancerender_filter_section_inheritance {
() => {
// Module: crate::renderer::tests::inheritance
// Provides: {"render_filter_section_inheritance"}
// Dependencies: {}
# [test] fn render_filter_section_inheritance () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("top" , "{% filter upper %}hello {% block main %}top{% endblock main %}{% endfilter %}") , ("bottom" , "{% extends 'top' %}{% block main %}bottom{% endblock %}") ,]) . unwrap () ; let result = tera . render ("bottom" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "HELLO BOTTOM" . to_string ()) ; }
};
}
