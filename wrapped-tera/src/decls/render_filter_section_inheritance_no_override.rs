macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! render_filter_section_inheritance_no_override {
    () => {
        deps!();
        # [test] fn render_filter_section_inheritance_no_override () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("top" , "{% filter upper %}hello {% block main %}top{% endblock main %}{% endfilter %}") , ("bottom" , "{% extends 'top' %}") ,]) . unwrap () ; let result = tera . render ("bottom" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "HELLO TOP" . to_string ()) ; }
    };
}

render_filter_section_inheritance_no_override!();