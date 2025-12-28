macro_rules! lex_basic_expr {
    () => {
        # [test] fn lex_basic_expr () { let inputs = vec ! ["admin" , "true" , "macros::something()" , "something()" , "a is defined" , "a is defined(2)" , "1 + 1" , "1 + counts" , "1 + counts.first" , "1 + 2 + 3 * 9/2 + 2.1" , "(1 + 2 + 3) * 9/2 + 2.1" , "10 * 2 % 5" ,] ; for i in inputs { assert_lex_rule ! (Rule :: basic_expr , i) ; } }
    };
}

lex_basic_expr!()