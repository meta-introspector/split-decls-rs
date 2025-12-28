macro_rules! unterminated_string {
    () => {
        # [test] fn unterminated_string () { assert_err_msg (r#"{{ "hey }}"# , & ["1:4" , "expected a value that can be negated"]) ; }
    };
}

unterminated_string!()