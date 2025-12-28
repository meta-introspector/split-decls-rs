macro_rules! invalid_test_argument {
    () => {
        # [test] fn invalid_test_argument () { assert_err_msg (r#"{% if a is odd(key=1) %}"# , & ["1:19" , "expected `or`, `and`, `not`, `<=`, `>=`, `<`, `>`, `==`, `!=`, `+`, `-`, `*`, `/`, `%`, or a filter"] ,) ; }
    };
}

invalid_test_argument!();