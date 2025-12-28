macro_rules! lex_string {
    () => {
        # [test] fn lex_string () { let inputs = vec ! ["\"Blabla\"" , "\"123\"" , "\'123\'" , "\'This is still a string\'" , "`this is backquted`" , "`and this too`" ,] ; for i in inputs { assert_lex_rule ! (Rule :: string , i) ; } }
    };
}

lex_string!();