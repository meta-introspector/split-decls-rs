macro_rules! invalid_macro_content {
    () => {
        # [test] fn invalid_macro_content () { assert_err_msg (r#"
{% macro input(label, type) %}
    {% macro nested() %}
    {% endmacro nested %}
{% endmacro input %}
    "# , & ["3:5" , "unexpected tag; expected `{% endmacro %}` or the macro content"] ,) ; }
    };
}

invalid_macro_content!();