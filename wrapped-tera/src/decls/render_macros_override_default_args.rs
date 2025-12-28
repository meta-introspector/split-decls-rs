macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! render_macros_override_default_args {
    () => {
        deps!();
        # [test] fn render_macros_override_default_args () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello(val=1) %}{{val}}{% endmacro hello %}") , ("hello.html" , "{% import \"macros\" as macros %}{{macros::hello(val=2)}}") ,]) . unwrap () ; let result = tera . render ("hello.html" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "2" . to_string ()) ; }
    };
}

render_macros_override_default_args!();