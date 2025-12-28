macro_rules! lex_block_tag {
    () => {
        # [test] fn lex_block_tag () { let inputs = vec ! ["{% block tag %}" , "{% block my_block %}"] ; for i in inputs { assert_lex_rule ! (Rule :: block_tag , i) ; } }
    };
}

lex_block_tag!()