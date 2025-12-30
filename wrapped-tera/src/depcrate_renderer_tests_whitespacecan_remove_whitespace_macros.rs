// Generated macro for can_remove_whitespace_macros (function)
macro_rules! Depcrate_renderer_tests_whitespacecan_remove_whitespace_macros {
() => {
// Module: crate::renderer::tests::whitespace
// Provides: {"can_remove_whitespace_macros"}
// Dependencies: {}
# [test] fn can_remove_whitespace_macros () { let mut context = Context :: new () ; context . insert ("numbers" , & vec ! [1 , 2 , 3]) ; let inputs = vec ! [(r#" {%- import "macros" as macros -%} {{macros::hey()}}"# , "Hey!") , (r#" {% import "macros" as macros %} {{macros::hey()}}"# , "Hey!") , (r#" {%- import "macros" as macros %} {%- set hey = macros::hey() -%} {{hey}}"# , "Hey!") ,] ; for (input , expected) in inputs { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros" , "{% macro hey() -%} Hey! {%- endmacro %}") , ("tpl" , input) ,]) . unwrap () ; assert_eq ! (tera . render ("tpl" , & context) . unwrap () , expected) ; } }
};
}
