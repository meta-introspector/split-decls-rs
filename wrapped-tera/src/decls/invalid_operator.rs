macro_rules! invalid_operator {
    () => {
        # [test] fn invalid_operator () { assert_err_msg ("{{ hey =! }}" , & ["1:8" , "expected `or`, `and`, `not`, `<=`, `>=`, `<`, `>`, `==`, `!=`, `+`, `-`, `*`, `/`, `%`, a filter, or a variable end (`}}`)"] ,) ; }
    };
}

invalid_operator!();