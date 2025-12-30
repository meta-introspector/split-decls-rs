// Generated macro for invalid_else (function)
macro_rules! Depcrate_parser_tests_errorsinvalid_else {
() => {
// Module: crate::parser::tests::errors
// Provides: {"invalid_else"}
// Dependencies: {}
# [test] fn invalid_else () { assert_err_msg (r#"
{% if true %}
{% else %}
{% else %}
{% endif %}
    "# , & ["4:1" , "unexpected tag; expected an endif tag (`{% endif %}`) or some content"] ,) ; }
};
}
