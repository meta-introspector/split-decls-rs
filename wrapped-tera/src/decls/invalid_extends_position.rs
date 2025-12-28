macro_rules! invalid_extends_position {
    () => {
        # [test] fn invalid_extends_position () { assert_err_msg (r#"
hello
{% extends "hey.html" %}
    "# , & ["3:1" , "unexpected tag; expected end of input, a macro definition tag (`{% macro my_macro() %}`, or some content"] ,) ; }
    };
}

invalid_extends_position!();