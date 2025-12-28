macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! render_multiple_inheritance {
    () => {
        deps!();
        # [test] fn render_multiple_inheritance () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("top" , "{% block pre %}{% endblock pre %}{% block main %}{% endblock main %}") , ("mid" , "{% extends \"top\" %}{% block pre %}PRE{% endblock pre %}") , ("bottom" , "{% extends \"mid\" %}{% block main %}MAIN{% endblock main %}") ,]) . unwrap () ; let result = tera . render ("bottom" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "PREMAIN" . to_string ()) ; }
    };
}

render_multiple_inheritance!();