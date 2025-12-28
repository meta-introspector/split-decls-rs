macro_rules! lex_if_tag {
    () => {
        # [test] fn lex_if_tag () { let inputs = vec ! ["{%- if name %}" , "{% if true -%}" , "{% if admin or show %}" , "{% if 1 + 2 == 2 and true %}" , "{% if 1 + 2 == 2 and admin is defined %}" ,] ; for i in inputs { assert_lex_rule ! (Rule :: if_tag , i) ; } }
    };
}

lex_if_tag!()