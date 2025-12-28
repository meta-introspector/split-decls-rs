macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! can_load_macro_in_child {
    () => {
        deps!();
        # [test] fn can_load_macro_in_child () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello()%}{{ 1 }}{% endmacro hello %}") , ("parent" , "{% block bob %}{% endblock bob %}") , ("child" , "{% extends \"parent\" %}{% import \"macros\" as macros %}{% block bob %}{{ macros::hello() }}{% endblock bob %}") ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "1" . to_string ()) ; }
    };
}

can_load_macro_in_child!();