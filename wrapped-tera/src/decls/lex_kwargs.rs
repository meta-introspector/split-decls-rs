macro_rules! lex_kwargs {
    () => {
        # [test] fn lex_kwargs () { let inputs = vec ! ["hello=1" , "hello=1+1,hey=1" , "hello1=true,name=name,admin=true" , "hello=name" , "hello=name|filter,id=1" , "hello=name|filter(with_arg=true),id=1" ,] ; for i in inputs { assert_lex_rule ! (Rule :: kwargs , i) ; } }
    };
}

lex_kwargs!()