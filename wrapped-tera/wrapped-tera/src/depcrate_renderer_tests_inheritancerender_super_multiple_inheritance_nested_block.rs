// Generated macro for render_super_multiple_inheritance_nested_block (function)
macro_rules! Depcrate_renderer_tests_inheritancerender_super_multiple_inheritance_nested_block {
() => {
// Module: crate::renderer::tests::inheritance
// Provides: {"render_super_multiple_inheritance_nested_block"}
// Dependencies: {}
# [test] fn render_super_multiple_inheritance_nested_block () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("grandparent" , "{% block hey %}hello{% endblock hey %}" ,) , ("parent" , "{% extends \"grandparent\" %}{% block hey %}hi and grandma says {{ super() }} {% block ending %}sincerely{% endblock ending %}{% endblock hey %}" ,) , ("child" , "{% extends \"parent\" %}{% block hey %}dad says {{ super() }}{% endblock hey %}{% block ending %}{{ super() }} with love{% endblock ending %}" ,) ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "dad says hi and grandma says hello sincerely with love" . to_string ()) ; }
};
}
