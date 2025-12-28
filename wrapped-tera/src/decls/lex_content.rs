macro_rules! lex_content {
    () => {
        # [test] fn lex_content () { let inputs = vec ! ["some text" , "{{ name }}" , "{# comment #}" , "{% filter upper %}hey{% endfilter %}" , "{% filter upper() %}hey{% endfilter %}" , "{% raw %}{{ hey }}{% endraw %}" , "{% for a in b %}{{a}}{% endfor %}" , "{% if i18n %}世界{% else %}world{% endif %}" ,] ; for i in inputs { assert_lex_rule ! (Rule :: content , i) ; } }
    };
}

lex_content!()