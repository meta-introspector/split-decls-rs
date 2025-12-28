macro_rules! missing_value_in_set {
    () => {
        # [test] fn missing_value_in_set () { assert_err_msg ("{% set a =  %}" , & ["1:13" , "expected a value that can be negated or an array of values"] ,) ; }
    };
}

missing_value_in_set!()