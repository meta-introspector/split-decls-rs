macro_rules! lex_basic_expr_with_filter {
    () => {
        # [test] fn lex_basic_expr_with_filter () { let inputs = vec ! ["admin | hello" , "true | ho" , "macros::something() | hey" , "something() | hey" , "a is defined | ho" , "a is defined(2) | ho" , "1 + 1 | round" , "1 + counts | round" , "1 + counts.first | round" , "1 + 2 + 3 * 9/2 + 2.1 | round" , "(1 + 2 + 3) * 9/2 + 2.1 | round" , "10 * 2 % 5 | round" ,] ; for i in inputs { assert_lex_rule ! (Rule :: basic_expr_filter , i) ; } }
    };
}

lex_basic_expr_with_filter!()