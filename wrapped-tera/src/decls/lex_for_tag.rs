macro_rules! lex_for_tag {
    () => {
        # [test] fn lex_for_tag () { let inputs = vec ! ["{%- for a in array %}" , "{% for a, b in object -%}" , "{% for a, b in fn_call() %}" , "{% for a in fn_call() %}" , "{% for a in [] %}" , "{% for a in [1,2,3,] %}" , "{% for a,b in fn_call(with_args=true, name=name) %}" , "{% for client in clients | slice(start=1, end=9) %}" ,] ; for i in inputs { assert_lex_rule ! (Rule :: for_tag , i) ; } }
    };
}

lex_for_tag!()