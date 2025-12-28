macro_rules! deps {
    () => {
        TeraParser!();
    };
}

macro_rules! lex_test {
    () => {
        deps!();
        # [test] fn lex_test () { let inputs = vec ! ["a is defined" , "a is defined()" , "a is divisibleby(2)" , "a is in([1, 2, something])"] ; for i in inputs { assert ! (TeraParser :: parse (Rule :: test , i) . is_ok ()) ; } }
    };
}

lex_test!();