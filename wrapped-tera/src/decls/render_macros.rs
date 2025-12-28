macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! render_macros {
    () => {
        deps!();
        # [test] fn render_macros () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello()%}Hello{% endmacro hello %}") , ("tpl" , "{% import \"macros\" as macros %}{% block hey %}{{macros::hello()}}{% endblock hey %}" ,) ,]) . unwrap () ; let result = tera . render ("tpl" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "Hello" . to_string ()) ; }
    };
}

render_macros!()