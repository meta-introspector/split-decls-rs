macro_rules! invalid_number {
    () => {
        # [test] fn invalid_number () { assert_err_msg ("{{ 1.2.2 }}" , & ["1:7" , "expected `or`, `and`, `not`, `<=`, `>=`, `<`, `>`, `==`, `!=`, `+`, `-`, `*`, `/`, `%`, a filter, or a variable end (`}}`)"] ,) ; }
    };
}

invalid_number!();