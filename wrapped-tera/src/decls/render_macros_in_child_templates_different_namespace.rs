macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! render_macros_in_child_templates_different_namespace {
    () => {
        deps!();
        # [test] fn render_macros_in_child_templates_different_namespace () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("grandparent" , "{% block hey %}hello{% endblock hey %}") , ("macros" , "{% macro hello()%}Hello{% endmacro hello %}") , ("macros2" , "{% macro hi()%}Hi{% endmacro hi %}") , ("parent" , "{% extends \"grandparent\" %}{% import \"macros\" as macros %}{% block hey %}{{macros::hello()}}{% endblock hey %}") , ("child" , "{% extends \"parent\" %}{% import \"macros2\" as macros2 %}{% block hey %}{{super()}}/{{macros2::hi()}}{% endblock hey %}") ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "Hello/Hi" . to_string ()) ; }
    };
}

render_macros_in_child_templates_different_namespace!();