macro_rules! lex_filter {
    () => {
        # [test] fn lex_filter () { let inputs = vec ! ["|attr" , "|attr()" , "|attr(key=1)" , "|attr(key=1, more=true)" , "|attr(key=1,more=true)" ,] ; for i in inputs { assert_lex_rule ! (Rule :: filter , i) ; } }
    };
}

lex_filter!()