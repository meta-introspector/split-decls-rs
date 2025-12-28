macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! macro_param_arent_escaped {
    () => {
        deps!();
        # [test] fn macro_param_arent_escaped () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros.html" , r#"{% macro print(val) %}{{val|safe}}{% endmacro print %}"#) , ("hello.html" , r#"{% import "macros.html" as macros %}{{ macros::print(val=my_var)}}"#) ,]) . unwrap () ; let mut context = Context :: new () ; context . insert ("my_var" , & "&") ; let result = tera . render ("hello.html" , & context) ; assert_eq ! (result . unwrap () , "&" . to_string ()) ; }
    };
}

macro_param_arent_escaped!();