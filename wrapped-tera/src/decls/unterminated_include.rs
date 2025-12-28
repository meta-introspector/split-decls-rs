macro_rules! unterminated_include {
    () => {
        # [test] fn unterminated_include () { assert_err_msg ("{% include %}" , & ["1:12" , "expected a string"]) ; }
    };
}

unterminated_include!();