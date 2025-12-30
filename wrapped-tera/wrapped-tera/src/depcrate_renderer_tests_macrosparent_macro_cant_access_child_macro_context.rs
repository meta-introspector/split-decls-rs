// Generated macro for parent_macro_cant_access_child_macro_context (function)
macro_rules! Depcrate_renderer_tests_macrosparent_macro_cant_access_child_macro_context {
() => {
// Module: crate::renderer::tests::macros
// Provides: {"parent_macro_cant_access_child_macro_context"}
// Dependencies: {}
# [test] fn parent_macro_cant_access_child_macro_context () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("parent" , "{% import \"macros\" as macros %}{{ macros::test_global() }}") , ("macros" , r#"{% import "moremacros" as moremacros %}{% macro test_global() %}{% set_global value1 = "ACAB" %}{{ moremacros::another_one() }}{{ value1 }}-{{ value2 | default(value="ACAB") }}{% endmacro test_global %}"#) , ("moremacros" , r#"{% macro another_one() %}{% set_global value2 = "1312" %}{% endmacro another_one %}"#)]) . unwrap () ; let result = tera . render ("parent" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "ACAB-ACAB" . to_string ()) ; }
};
}
