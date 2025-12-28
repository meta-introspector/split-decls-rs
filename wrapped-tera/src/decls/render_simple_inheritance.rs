macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! render_simple_inheritance {
    () => {
        deps!();
        # [test] fn render_simple_inheritance () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("top" , "{% block pre %}{% endblock pre %}{% block main %}{% endblock main %}") , ("bottom" , "{% extends \"top\" %}{% block main %}MAIN{% endblock %}") ,]) . unwrap () ; let result = tera . render ("bottom" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "MAIN" . to_string ()) ; }
    };
}

render_simple_inheritance!();