macro_rules! invalid_elif {
    () => {
        # [test] fn invalid_elif () { assert_err_msg (r#"
{% if true %}
{% else %}
{% elif false %}
{% endif %}
    "# , & ["4:1" , "unexpected tag; expected an endif tag (`{% endif %}`) or some content"] ,) ; }
    };
}

invalid_elif!()