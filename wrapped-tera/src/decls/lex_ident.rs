macro_rules! deps {
    () => {
        TeraParser!();
    };
}

macro_rules! lex_ident {
    () => {
        deps!();
        # [test] fn lex_ident () { let inputs = vec ! ["hello" , "hello_" , "hello_1" , "HELLO" , "_1"] ; for i in inputs { assert_lex_rule ! (Rule :: ident , i) ; } assert ! (TeraParser :: parse (Rule :: ident , "909") . is_err ()) ; }
    };
}

lex_ident!();