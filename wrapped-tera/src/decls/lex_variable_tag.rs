macro_rules! lex_variable_tag {
    () => {
        # [test] fn lex_variable_tag () { let inputs = vec ! ["{{ a }}" , "{{ a | caps }}" , r#"{{ "hey" }}"# , r#"{{ 'hey' }}"# , r#"{{ `hey` }}"# , "{{ fn_call() }}" , "{{ macros::fn() }}" , "{{ name + 42 }}" , "{{ loop.index + 1 }}" , "{{ name is defined and name >= 42 }}" , "{{ my_macros::macro1(hello=\"world\", foo=bar, hey=1+2) }}" , "{{ 'hello' ~ `ho` }}" , r#"{{ hello ~ `ho` }}"# ,] ; for i in inputs { assert_lex_rule ! (Rule :: variable_tag , i) ; } }
    };
}

lex_variable_tag!()