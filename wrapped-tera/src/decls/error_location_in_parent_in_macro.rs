macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! error_location_in_parent_in_macro {
    () => {
        deps!();
        # [test] fn error_location_in_parent_in_macro () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello()%}{{ 1 + true }}{% endmacro hello %}") , ("parent" , "{% import \"macros\" as macros %}{{ macros::hello() }}{% block bob %}{% endblock bob %}") , ("child" , "{% extends \"parent\" %}{% block bob %}{{ super() }}Hey{% endblock bob %}") ,]) . unwrap () ; let result = tera . render ("child" , & Context :: new ()) ; assert_eq ! (result . unwrap_err () . to_string () , "Failed to render \'child\': error while rendering macro `macros::hello` (error happened in \'parent\').") ; }
    };
}

error_location_in_parent_in_macro!()