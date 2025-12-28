macro_rules! invalid_include_no_string {
    () => {
        # [test] fn invalid_include_no_string () { assert_err_msg ("{% include 1 %}" , & ["1:12" , "expected a string"]) ; }
    };
}

invalid_include_no_string!();