macro_rules! invalid_block_missing_name {
    () => {
        # [test] fn invalid_block_missing_name () { assert_err_msg (r#"{% block %}"# , & ["1:10" , "expected an identifier (must start with a-z)"]) ; }
    };
}

invalid_block_missing_name!()