macro_rules! missing_container_name_in_forloop {
    () => {
        # [test] fn missing_container_name_in_forloop () { assert_err_msg ("{% for i in %}" , & ["1:13" , "expected an expression or an array of values"]) ; }
    };
}

missing_container_name_in_forloop!();