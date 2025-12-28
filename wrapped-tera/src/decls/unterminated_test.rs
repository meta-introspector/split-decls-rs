macro_rules! unterminated_test {
    () => {
        # [test] fn unterminated_test () { assert_err_msg (r#"{% if a is odd( %}"# , & ["1:17" , "a test argument (any expressions including arrays)"] ,) ; }
    };
}

unterminated_test!()