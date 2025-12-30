// Generated macro for render_multiple_inheritance_with_super (function)
macro_rules! Depcrate_renderer_tests_inheritancerender_multiple_inheritance_with_super {
() => {
// Module: crate::renderer::tests::inheritance
// Provides: {"render_multiple_inheritance_with_super"}
// Dependencies: {}
# [test] fn render_multiple_inheritance_with_super () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("grandparent" , "{% block hey %}hello{% endblock hey %} {% block ending %}sincerely{% endblock ending %}" ,) , ("parent" , "{% extends \"grandparent\" %}{% block hey %}hi and grandma says {{ super() }}{% endblock hey %}" ,) , ("child" , "{% extends \"parent\" %}{% block hey %}dad says {{ super() }}{% endblock hey %}{% block ending %}{{ super() }} with love{% endblock ending %}" ,) ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "dad says hi and grandma says hello sincerely with love" . to_string ()) ; }
};
}
