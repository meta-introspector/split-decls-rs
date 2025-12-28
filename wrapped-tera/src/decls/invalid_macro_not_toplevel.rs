macro_rules! invalid_macro_not_toplevel {
    () => {
        # [test] fn invalid_macro_not_toplevel () { assert_err_msg (r#"
{% if val %}
    {% macro input(label, type) %}
    {% endmacro input %}
{% endif %}
    "# , & ["3:5" , "unexpected tag; expected an `elif` tag, an `else` tag, an endif tag (`{% endif %}`), or some content"] ,) ; }
    };
}

invalid_macro_not_toplevel!()