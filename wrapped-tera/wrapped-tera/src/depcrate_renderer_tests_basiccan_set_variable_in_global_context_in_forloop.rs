// Generated macro for can_set_variable_in_global_context_in_forloop (function)
macro_rules! Depcrate_renderer_tests_basiccan_set_variable_in_global_context_in_forloop {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"can_set_variable_in_global_context_in_forloop"}
// Dependencies: {}
# [test] fn can_set_variable_in_global_context_in_forloop () { let mut context = Context :: new () ; context . insert ("tags" , & vec ! [1 , 2 , 3]) ; context . insert ("default" , & "default") ; let result = render_template (r#"
{%- for i in tags -%}
{%- set default = 1 -%}
{%- set_global global_val = i -%}
{%- endfor -%}
{{ default }}{{ global_val }}"# , & context ,) ; assert_eq ! (result . unwrap () , "default3") ; }
};
}
