macro_rules! lex_fn_call {
    () => {
        # [test] fn lex_fn_call () { let inputs = vec ! ["fn(hello=1)" , "fn(hello=1+1,hey=1)" , "fn(hello1=true,name=name,admin=true)" , "fn(hello=name)" , "fn(hello=name,)" , "fn(\n  hello=name,\n)" , "fn(hello=name|filter,id=1)" ,] ; for i in inputs { assert_lex_rule ! (Rule :: fn_call , i) ; } }
    };
}

lex_fn_call!()