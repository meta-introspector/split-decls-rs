macro_rules! missing_expression_with_not {
    () => {
        # [test] fn missing_expression_with_not () { assert_err_msg ("{% if not %}" , & ["1:11" , "expected an expression"]) ; }
    };
}

missing_expression_with_not!();