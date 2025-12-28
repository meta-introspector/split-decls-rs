macro_rules! invalid_extends_no_string {
    () => {
        # [test] fn invalid_extends_no_string () { assert_err_msg ("{% extends 1 %}" , & ["1:12" , "expected a string"]) ; }
    };
}

invalid_extends_no_string!()