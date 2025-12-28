macro_rules! lex_array {
    () => {
        # [test] fn lex_array () { let inputs = vec ! ["[]" , "[1,2,3]" , "[1, 2,3,]" , "[1 + 1, 2,3 * 2,]" , "[\"foo\", \"bar\"]" , "[1,true,'string', 0.5, hello(), macros::hey(arg=1)]" ,] ; for i in inputs { assert_lex_rule ! (Rule :: array , i) ; } }
    };
}

lex_array!();