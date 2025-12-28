macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! render_multiple_inheritance_with_super {
    () => {
        deps!();
        # [test] fn render_multiple_inheritance_with_super () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("grandparent" , "{% block hey %}hello{% endblock hey %} {% block ending %}sincerely{% endblock ending %}" ,) , ("parent" , "{% extends \"grandparent\" %}{% block hey %}hi and grandma says {{ super() }}{% endblock hey %}" ,) , ("child" , "{% extends \"parent\" %}{% block hey %}dad says {{ super() }}{% endblock hey %}{% block ending %}{{ super() }} with love{% endblock ending %}" ,) ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "dad says hi and grandma says hello sincerely with love" . to_string ()) ; }
    };
}

render_multiple_inheritance_with_super!()