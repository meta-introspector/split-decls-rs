macro_rules! lex_in_cond {
    () => {
        # [test] fn lex_in_cond () { let inputs = vec ! ["a in b" , "1 in b" , "'b' in b" , "'b' in b" , "a in request.path" , "'index.html' in request.build_absolute_uri" , "a in [1, 2, 3]" , "a | capitalize in [1, 2, 3]" , "a | capitalize in [1, 'hey']" , "a | capitalize in [ho, 1, 'hey']" , "'e' in 'hello'" , "'e' in 'hello' | capitalize" , "e in 'hello'" ,] ; for i in inputs { assert_lex_rule ! (Rule :: in_cond , i) ; } }
    };
}

lex_in_cond!()