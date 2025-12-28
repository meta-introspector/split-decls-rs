macro_rules! missing_expression_in_if {
    () => {
        # [test] fn missing_expression_in_if () { assert_err_msg ("{% if %}" , & ["1:7" , "expected a value that can be negated"]) ; }
    };
}

missing_expression_in_if!();