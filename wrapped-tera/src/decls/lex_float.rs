macro_rules! lex_float {
    () => {
        # [test] fn lex_float () { let inputs = vec ! ["123.5" , "123.5" , "0.1" , "-1.1"] ; for i in inputs { assert_lex_rule ! (Rule :: float , i) ; } }
    };
}

lex_float!()