macro_rules! invalid_else {
    () => {
        # [test] fn invalid_else () { assert_err_msg (r#"
{% if true %}
{% else %}
{% else %}
{% endif %}
    "# , & ["4:1" , "unexpected tag; expected an endif tag (`{% endif %}`) or some content"] ,) ; }
    };
}

invalid_else!()