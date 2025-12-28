macro_rules! missing_variable_name_in_set {
    () => {
        # [test] fn missing_variable_name_in_set () { assert_err_msg ("{% set = 1 %}" , & ["1:8" , "expected an identifier (must start with a-z)"]) ; }
    };
}

missing_variable_name_in_set!()