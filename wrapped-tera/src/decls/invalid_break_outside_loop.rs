macro_rules! invalid_break_outside_loop {
    () => {
        # [test] fn invalid_break_outside_loop () { assert_err_msg (r#"{% break %}"# , & ["1:1" , "{% break %}" , "expected a template"]) ; }
    };
}

invalid_break_outside_loop!()