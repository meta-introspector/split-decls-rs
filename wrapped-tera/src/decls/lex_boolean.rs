macro_rules! lex_boolean {
    () => {
        # [test] fn lex_boolean () { let inputs = vec ! ["true" , "false" , "True" , "False"] ; for i in inputs { assert_lex_rule ! (Rule :: boolean , i) ; } }
    };
}

lex_boolean!();