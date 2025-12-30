// Generated macro for right_variable_name_is_needed_in_for_loop (function)
macro_rules! Depcrate_renderer_tests_errorsright_variable_name_is_needed_in_for_loop {
() => {
// Module: crate::renderer::tests::errors
// Provides: {"right_variable_name_is_needed_in_for_loop"}
// Dependencies: {}
# [test] fn right_variable_name_is_needed_in_for_loop () { let mut data = HashMap :: new () ; data . insert ("content" , "hello") ; let mut context = Context :: new () ; context . insert ("comments" , & vec ! [data]) ; let mut tera = Tera :: default () ; tera . add_raw_template ("tpl" , r#"
{%- for comment in comments -%}
<p>{{ comment.content }}</p>
<p>{{ whocares.content }}</p>
<p>{{ doesntmatter.content }}</p>
{% endfor -%}"# ,) . unwrap () ; let result = tera . render ("tpl" , & context) ; assert_eq ! (result . unwrap_err () . source () . unwrap () . to_string () , "Variable `whocares.content` not found in context while rendering \'tpl\'") ; }
};
}
