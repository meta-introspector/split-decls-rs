macro_rules! lex_set_global_tag {
    () => {
        # [test] fn lex_set_global_tag () { let inputs = vec ! ["{% set_global a = 1 %}" , "{% set_global a = [1,2,3, 'hey'] -%}" , "{% set_global a = another_var %}" , "{% set_global a = another_var | filter %}" , "{% set_global a = var +1 >= 2%}" , "{%- set_global a = var +1 >= 2 -%}" ,] ; for i in inputs { assert_lex_rule ! (Rule :: set_global_tag , i) ; } }
    };
}

lex_set_global_tag!()