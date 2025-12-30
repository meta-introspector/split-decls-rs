// Generated macro for render_macros_in_parent_template_with_inheritance (function)
macro_rules! Depcrate_renderer_tests_macrosrender_macros_in_parent_template_with_inheritance {
() => {
// Module: crate::renderer::tests::macros
// Provides: {"render_macros_in_parent_template_with_inheritance"}
// Dependencies: {}
# [test] fn render_macros_in_parent_template_with_inheritance () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello()%}Hello{% endmacro hello %}") , ("grandparent" , "{% import \"macros\" as macros %}{% block hey %}{{macros::hello()}}{% endblock hey %}") , ("child" , "{% extends \"grandparent\" %}{% import \"macros\" as macros %}{% block hey %}{{super()}}/{{macros::hello()}}{% endblock hey %}") ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "Hello/Hello" . to_string ()) ; }
};
}
