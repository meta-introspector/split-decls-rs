// Generated macro for invalid_elif (function)
macro_rules! Depcrate_parser_tests_errorsinvalid_elif {
() => {
// Module: crate::parser::tests::errors
// Provides: {"invalid_elif"}
// Dependencies: {}
# [test] fn invalid_elif () { assert_err_msg (r#"
{% if true %}
{% else %}
{% elif false %}
{% endif %}
    "# , & ["4:1" , "unexpected tag; expected an endif tag (`{% endif %}`) or some content"] ,) ; }
};
}
