macro_rules! unterminated_macro_call {
    () => {
        # [test] fn unterminated_macro_call () { assert_err_msg ("{{ my::macro( }}" , & ["1:15" , "expected an identifier (must start with a-z)"]) ; }
    };
}

unterminated_macro_call!()