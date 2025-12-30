// Generated macro for render_super_in_grandchild_without_redefining_in_parent_works (function)
macro_rules! Depcrate_renderer_tests_inheritancerender_super_in_grandchild_without_redefining_in_parent_works {
() => {
// Module: crate::renderer::tests::inheritance
// Provides: {"render_super_in_grandchild_without_redefining_in_parent_works"}
// Dependencies: {}
# [test] fn render_super_in_grandchild_without_redefining_in_parent_works () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("grandparent" , "{% block title %}Title{% endblock %}") , ("parent" , "{% extends \"grandparent\" %}") , ("child" , "{% extends \"parent\" %}{% block title %}{{ super() }} - More{% endblock %}") ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "Title - More" . to_string ()) ; }
};
}
