macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! render_macros_in_parent_template_with_inheritance {
    () => {
        deps!();
        # [test] fn render_macros_in_parent_template_with_inheritance () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello()%}Hello{% endmacro hello %}") , ("grandparent" , "{% import \"macros\" as macros %}{% block hey %}{{macros::hello()}}{% endblock hey %}") , ("child" , "{% extends \"grandparent\" %}{% import \"macros\" as macros %}{% block hey %}{{super()}}/{{macros::hello()}}{% endblock hey %}") ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "Hello/Hello" . to_string ()) ; }
    };
}

render_macros_in_parent_template_with_inheritance!()