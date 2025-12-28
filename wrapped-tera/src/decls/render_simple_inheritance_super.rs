macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! render_simple_inheritance_super {
    () => {
        deps!();
        # [test] fn render_simple_inheritance_super () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("top" , "{% block main %}TOP{% endblock main %}") , ("bottom" , "{% extends \"top\" %}{% block main %}{{ super() }}MAIN{% endblock %}") ,]) . unwrap () ; let result = tera . render ("bottom" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "TOPMAIN" . to_string ()) ; }
    };
}

render_simple_inheritance_super!()