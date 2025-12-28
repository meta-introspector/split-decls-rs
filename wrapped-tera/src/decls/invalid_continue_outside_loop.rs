macro_rules! invalid_continue_outside_loop {
    () => {
        # [test] fn invalid_continue_outside_loop () { assert_err_msg (r#"{% continue %}"# , & ["1:1" , "{% continue %}" , "expected a template"]) ; }
    };
}

invalid_continue_outside_loop!();