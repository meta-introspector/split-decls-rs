macro_rules! unterminated_variable_block {
    () => {
        # [test] fn unterminated_variable_block () { assert_err_msg ("{{ hey" , & ["1:7" , "expected `or`, `and`, `not`, `<=`, `>=`, `<`, `>`, `==`, `!=`, `+`, `-`, `*`, `/`, `%`, a filter, or a variable end (`}}`)"] ,) ; }
    };
}

unterminated_variable_block!()