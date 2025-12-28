macro_rules! invalid_macro_default_arg_value {
    () => {
        # [test] fn invalid_macro_default_arg_value () { assert_err_msg (r#"
{% macro input(label=something) %}
{% endmacro input %}
    "# , & ["2:22" , "expected an integer, a float, a string, or `true` or `false`"] ,) ; }
    };
}

invalid_macro_default_arg_value!()