macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! render_filter_section_inheritance {
    () => {
        deps!();
        # [test] fn render_filter_section_inheritance () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("top" , "{% filter upper %}hello {% block main %}top{% endblock main %}{% endfilter %}") , ("bottom" , "{% extends 'top' %}{% block main %}bottom{% endblock %}") ,]) . unwrap () ; let result = tera . render ("bottom" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "HELLO BOTTOM" . to_string ()) ; }
    };
}

render_filter_section_inheritance!()