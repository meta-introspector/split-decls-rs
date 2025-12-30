// Generated macro for invalid_macro_not_toplevel (function)
macro_rules! Depcrate_parser_tests_errorsinvalid_macro_not_toplevel {
() => {
// Module: crate::parser::tests::errors
// Provides: {"invalid_macro_not_toplevel"}
// Dependencies: {}
# [test] fn invalid_macro_not_toplevel () { assert_err_msg (r#"
{% if val %}
    {% macro input(label, type) %}
    {% endmacro input %}
{% endif %}
    "# , & ["3:5" , "unexpected tag; expected an `elif` tag, an `else` tag, an endif tag (`{% endif %}`), or some content"] ,) ; }
};
}
