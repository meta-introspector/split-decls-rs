macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! parent_macro_cant_access_child_macro_context {
    () => {
        deps!();
        # [test] fn parent_macro_cant_access_child_macro_context () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("parent" , "{% import \"macros\" as macros %}{{ macros::test_global() }}") , ("macros" , r#"{% import "moremacros" as moremacros %}{% macro test_global() %}{% set_global value1 = "ACAB" %}{{ moremacros::another_one() }}{{ value1 }}-{{ value2 | default(value="ACAB") }}{% endmacro test_global %}"#) , ("moremacros" , r#"{% macro another_one() %}{% set_global value2 = "1312" %}{% endmacro another_one %}"#)]) . unwrap () ; let result = tera . render ("parent" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "ACAB-ACAB" . to_string ()) ; }
    };
}

parent_macro_cant_access_child_macro_context!()