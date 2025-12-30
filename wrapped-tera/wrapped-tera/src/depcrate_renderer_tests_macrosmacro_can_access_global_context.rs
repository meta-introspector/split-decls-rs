// Generated macro for macro_can_access_global_context (function)
macro_rules! Depcrate_renderer_tests_macrosmacro_can_access_global_context {
() => {
// Module: crate::renderer::tests::macros
// Provides: {"macro_can_access_global_context"}
// Dependencies: {}
# [test] fn macro_can_access_global_context () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("parent" , r#"{% import "macros" as macros %}{{ macros::test_global() }}"#) , ("macros" , r#"{% macro test_global() %}{% set_global value1 = "42" %}{% for i in range(end=1) %}{% set_global value2 = " is the truth." %}{% endfor %}{{ value1 }}{% endmacro test_global %}"#)]) . unwrap () ; let result = tera . render ("parent" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "42" . to_string ()) ; }
};
}
