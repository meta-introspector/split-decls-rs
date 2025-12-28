macro_rules! lex_filter_tag {
    () => {
        # [test] fn lex_filter_tag () { let inputs = vec ! ["{%- filter tag() %}" , "{% filter foo(bar=baz) -%}" , "{% filter foo(bar=42) %}" , "{% filter foo(bar=baz,qux=quz) %}" , "{% filter foo(bar=baz, qux=quz) %}" , "{% filter foo ( bar=\"baz\", qux=42 ) %}" ,] ; for i in inputs { assert_lex_rule ! (Rule :: filter_tag , i) ; } }
    };
}

lex_filter_tag!()