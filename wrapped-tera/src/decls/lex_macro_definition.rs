macro_rules! deps {
    () => {
        TeraParser!();
    };
}

macro_rules! lex_macro_definition {
    () => {
        deps!();
        # [test] fn lex_macro_definition () { let inputs = vec ! ["hello()" , "hello(name, admin)" , "hello(name, admin=1)" , "hello(name=\"bob\", admin)" , "hello(name=\"bob\",admin=true)" ,] ; for i in inputs { assert ! (TeraParser :: parse (Rule :: macro_fn , i) . is_ok ()) ; } }
    };
}

lex_macro_definition!()