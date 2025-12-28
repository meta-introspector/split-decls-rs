macro_rules! wrong_end_block {
    () => {
        # [test] fn wrong_end_block () { assert_err_msg ("{{ hey %}" , & ["1:9" , "expected an integer, a float, `true` or `false`, an identifier (must start with a-z), a square bracketed identifier (identifiers separated by `.` or `[]`s), or an expression"] ,) ; }
    };
}

wrong_end_block!();