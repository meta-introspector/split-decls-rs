// Generated macro for invalid_macro_content (function)
macro_rules! Depcrate_parser_tests_errorsinvalid_macro_content {
() => {
// Module: crate::parser::tests::errors
// Provides: {"invalid_macro_content"}
// Dependencies: {}
# [test] fn invalid_macro_content () { assert_err_msg (r#"
{% macro input(label, type) %}
    {% macro nested() %}
    {% endmacro nested %}
{% endmacro input %}
    "# , & ["3:5" , "unexpected tag; expected `{% endmacro %}` or the macro content"] ,) ; }
};
}
