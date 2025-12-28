macro_rules! invalid_macro_call {
    () => {
        # [test] fn invalid_macro_call () { assert_err_msg ("{{ my:macro() }}" , & ["1:6" , "expected `or`, `and`, `not`, `<=`, `>=`, `<`, `>`, `==`, `!=`, `+`, `-`, `*`, `/`, `%`, a filter, or a variable end (`}}`)"] ,) ; }
    };
}

invalid_macro_call!()