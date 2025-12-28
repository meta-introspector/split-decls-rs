macro_rules! lex_macro_tag {
    () => {
        # [test] fn lex_macro_tag () { let inputs = vec ! ["{%- macro tag() %}" , "{% macro my_block(name) -%}" , "{% macro my_block(name=42) %}" , "{% macro foo ( bar=\"baz\", qux=42 ) %}" ,] ; for i in inputs { assert_lex_rule ! (Rule :: macro_tag , i) ; } }
    };
}

lex_macro_tag!();