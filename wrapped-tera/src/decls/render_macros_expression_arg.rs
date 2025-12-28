macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! render_macros_expression_arg {
    () => {
        deps!();
        # [test] fn render_macros_expression_arg () { let mut context = Context :: new () ; context . insert ("pages" , & vec ! [1 , 2 , 3 , 4 , 5]) ; let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hello(val)%}{{val}}{% endmacro hello %}") , ("tpl" , "{% import \"macros\" as macros %}{{macros::hello(val=pages|last)}}") ,]) . unwrap () ; let result = tera . render ("tpl" , & context) ; assert_eq ! (result . unwrap () , "5" . to_string ()) ; }
    };
}

render_macros_expression_arg!();