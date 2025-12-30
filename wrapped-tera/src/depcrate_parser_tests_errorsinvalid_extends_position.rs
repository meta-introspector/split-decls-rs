// Generated macro for invalid_extends_position (function)
macro_rules! Depcrate_parser_tests_errorsinvalid_extends_position {
() => {
// Module: crate::parser::tests::errors
// Provides: {"invalid_extends_position"}
// Dependencies: {}
# [test] fn invalid_extends_position () { assert_err_msg (r#"
hello
{% extends "hey.html" %}
    "# , & ["3:1" , "unexpected tag; expected end of input, a macro definition tag (`{% macro my_macro() %}`, or some content"] ,) ; }
};
}
