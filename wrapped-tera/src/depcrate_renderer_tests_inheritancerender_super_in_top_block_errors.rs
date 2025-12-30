// Generated macro for render_super_in_top_block_errors (function)
macro_rules! Depcrate_renderer_tests_inheritancerender_super_in_top_block_errors {
() => {
// Module: crate::renderer::tests::inheritance
// Provides: {"render_super_in_top_block_errors"}
// Dependencies: {}
# [test] fn render_super_in_top_block_errors () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("index" , "{% block content%}{{super()}}{% endblock content %}")]) . unwrap () ; let result = tera . render ("index" , & Context :: new ()) ; assert ! (result . is_err ()) ; }
};
}
