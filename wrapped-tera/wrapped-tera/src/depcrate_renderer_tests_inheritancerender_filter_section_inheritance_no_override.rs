// Generated macro for render_filter_section_inheritance_no_override (function)
macro_rules! Depcrate_renderer_tests_inheritancerender_filter_section_inheritance_no_override {
() => {
// Module: crate::renderer::tests::inheritance
// Provides: {"render_filter_section_inheritance_no_override"}
// Dependencies: {}
# [test] fn render_filter_section_inheritance_no_override () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("top" , "{% filter upper %}hello {% block main %}top{% endblock main %}{% endfilter %}") , ("bottom" , "{% extends 'top' %}") ,]) . unwrap () ; let result = tera . render ("bottom" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "HELLO TOP" . to_string ()) ; }
};
}
