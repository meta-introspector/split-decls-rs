macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! error_location_inside_macro {
    () => {
        deps!();
        # [test] fn error_location_inside_macro () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello()%}{{ 1 + true }}{% endmacro hello %}") , ("tpl" , "{% import \"macros\" as macros %}{{ macros::hello() }}") ,]) . unwrap () ; let result = tera . render ("tpl" , & Context :: new ()) ; assert_eq ! (result . unwrap_err () . to_string () , "Failed to render \'tpl\': error while rendering macro `macros::hello`") ; }
    };
}

error_location_inside_macro!();