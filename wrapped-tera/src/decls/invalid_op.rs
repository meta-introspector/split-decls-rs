macro_rules! invalid_op {
    () => {
        # [test] fn invalid_op () { assert_err_msg ("{{ 1.2 >+ 3 }}" , & ["1:9" , "expected an expression"]) ; }
    };
}

invalid_op!();