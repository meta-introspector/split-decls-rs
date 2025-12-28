macro_rules! lex_int {
    () => {
        # [test] fn lex_int () { let inputs = vec ! ["-10" , "0" , "100" , "250000"] ; for i in inputs { assert_lex_rule ! (Rule :: int , i) ; } }
    };
}

lex_int!()