// Generated macro for redefining_loop_value_doesnt_break_loop (function)
macro_rules! Depcrate_renderer_tests_basicredefining_loop_value_doesnt_break_loop {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"redefining_loop_value_doesnt_break_loop"}
// Dependencies: {}
# [test] fn redefining_loop_value_doesnt_break_loop () { let mut tera = Tera :: default () ; tera . add_raw_template ("tpl" , r#"
{%- set string = "abcdefghdijklm" | split(pat="d") -%}
{% for i in string -%}
    {%- set j = i ~ "lol" ~ " " -%}
    {{ j }}
{%- endfor -%}
        "# ,) . unwrap () ; let context = Context :: new () ; let result = tera . render ("tpl" , & context) ; assert_eq ! (result . unwrap () , "abclol efghlol ijklmlol ") ; }
};
}
