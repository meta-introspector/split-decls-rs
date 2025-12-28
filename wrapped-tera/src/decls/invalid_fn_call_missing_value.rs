macro_rules! invalid_fn_call_missing_value {
    () => {
        # [test] fn invalid_fn_call_missing_value () { assert_err_msg ("{{ a | slice(start=) }}" , & ["1:20" , "expected a value that can be negated or an array of values"] ,) ; }
    };
}

invalid_fn_call_missing_value!()