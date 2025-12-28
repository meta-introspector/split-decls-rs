macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! render_set_tag_macro {
    () => {
        deps!();
        # [test] fn render_set_tag_macro () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello()%}Hello{% endmacro hello %}") , ("hello.html" , "{% import \"macros\" as macros %}{% set my_var = macros::hello() %}{{my_var}}" ,) ,]) . unwrap () ; let result = tera . render ("hello.html" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "Hello" . to_string ()) ; }
    };
}

render_set_tag_macro!()