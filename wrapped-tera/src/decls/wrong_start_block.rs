macro_rules! wrong_start_block {
    () => {
        # [test] fn wrong_start_block () { assert_err_msg ("{{ if true %}" , & ["1:7" , "expected `or`, `and`, `not`, `<=`, `>=`, `<`, `>`, `==`, `!=`, `+`, `-`, `*`, `/`, `%`, a filter, or a variable end (`}}`)"] ,) ; }
    };
}

wrong_start_block!()