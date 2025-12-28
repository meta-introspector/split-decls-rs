macro_rules! unterminated_extends {
    () => {
        # [test] fn unterminated_extends () { assert_err_msg ("{% extends %}" , & ["1:12" , "expected a string"]) ; }
    };
}

unterminated_extends!();