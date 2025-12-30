// Generated macro for invalid_macro_default_arg_value (function)
macro_rules! Depcrate_parser_tests_errorsinvalid_macro_default_arg_value {
() => {
// Module: crate::parser::tests::errors
// Provides: {"invalid_macro_default_arg_value"}
// Dependencies: {}
# [test] fn invalid_macro_default_arg_value () { assert_err_msg (r#"
{% macro input(label=something) %}
{% endmacro input %}
    "# , & ["2:22" , "expected an integer, a float, a string, or `true` or `false`"] ,) ; }
};
}
