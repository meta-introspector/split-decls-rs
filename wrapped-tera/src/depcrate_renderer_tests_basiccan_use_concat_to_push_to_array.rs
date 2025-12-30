// Generated macro for can_use_concat_to_push_to_array (function)
macro_rules! Depcrate_renderer_tests_basiccan_use_concat_to_push_to_array {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"can_use_concat_to_push_to_array"}
// Dependencies: {}
# [test] fn can_use_concat_to_push_to_array () { let mut tera = Tera :: default () ; tera . add_raw_template ("tpl" , r#"
{%- set ids = [] -%}
{% for i in range(end=5) -%}
{%- set_global ids = ids | concat(with=i) -%}
{%- endfor -%}
{{ids}}"# ,) . unwrap () ; let context = Context :: new () ; let result = tera . render ("tpl" , & context) ; assert_eq ! (result . unwrap () , "[0, 1, 2, 3, 4]") ; }
};
}
