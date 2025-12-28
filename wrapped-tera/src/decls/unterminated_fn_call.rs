macro_rules! unterminated_fn_call {
    () => {
        # [test] fn unterminated_fn_call () { assert_err_msg ("{{ a | slice( }}" , & ["1:15" , "expected an identifier (must start with a-z)"]) ; }
    };
}

unterminated_fn_call!()