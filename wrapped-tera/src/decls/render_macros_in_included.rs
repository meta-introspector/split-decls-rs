macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! render_macros_in_included {
    () => {
        deps!();
        # [test] fn render_macros_in_included () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro my_macro() %}my macro{% endmacro %}") , ("includeme" , r#"{% import "macros" as macros %}{{ macros::my_macro() }}"#) , ("example" , r#"{% include "includeme" %}"#) ,]) . unwrap () ; let result = tera . render ("example" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "my macro" . to_string ()) ; }
    };
}

render_macros_in_included!();