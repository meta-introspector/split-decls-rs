macro_rules! lex_comparison_expr {
    () => {
        # [test] fn lex_comparison_expr () { let inputs = vec ! ["1.5 + a | round(var=2) > 10" , "1.5 + a | round(var=2) > a | round" , "a == b" , "a + 1 == b" , "a != b" , "a % 2 == 0" , "a == 'admin'" , "a != 'admin'" , "a == 'admin' | capitalize" , "a != 'admin' | capitalize" , "a > b" , "a >= b" , "a < b" , "a <= b" , "true > false" ,] ; for i in inputs { assert_lex_rule ! (Rule :: comparison_expr , i) ; } }
    };
}

lex_comparison_expr!();