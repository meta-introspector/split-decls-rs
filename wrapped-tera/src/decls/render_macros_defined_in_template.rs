macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! render_macros_defined_in_template {
    () => {
        deps!();
        # [test] fn render_macros_defined_in_template () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("tpl" , "{% macro hello()%}Hello{% endmacro hello %}{% block hey %}{{self::hello()}}{% endblock hey %}") ,]) . unwrap () ; let result = tera . render ("tpl" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "Hello" . to_string ()) ; }
    };
}

render_macros_defined_in_template!();