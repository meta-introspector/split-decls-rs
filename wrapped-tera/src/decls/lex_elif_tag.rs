macro_rules! lex_elif_tag {
    () => {
        # [test] fn lex_elif_tag () { let inputs = vec ! ["{%- elif name %}" , "{% elif true -%}" , "{% elif admin or show %}" , "{% elif 1 + 2 == 2 and true %}" , "{% elif 1 + 2 == 2 and admin is defined %}" ,] ; for i in inputs { assert_lex_rule ! (Rule :: elif_tag , i) ; } }
    };
}

lex_elif_tag!()