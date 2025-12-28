macro_rules! deps {
    () => {
        TeraParser!();
    };
}

macro_rules! lex_requires_whitespace_between_things {
    () => {
        deps!();
        # [test] fn lex_requires_whitespace_between_things () { let inputs = vec ! ["{% filterupper %}hey{% endfilter %}" , "{% blockhey %}{%endblock%}" , "{% macrohey() %}{%endmacro%}" , "{% setident = 1 %}" , "{% set_globalident = 1 %}" , "{% extends'base.html' %}" , "{% import 'macros/image.html' asimage %}" , "{% import'macros/image.html' as image %}" , "{% fora in b %}{{a}}{% endfor %}" , "{% for a inb %}{{a}}{% endfor %}" , "{% for a,bin c %}{{a}}{% endfor %}" , "{% for a,b inc %}{{a}}{% endfor %}" , "{% ifi18n %}世界{% else %}world{% endif %}" , "{% if i18n %}世界{% eliftrue %}world{% endif %}" , "{% include'base.html' %}" ,] ; for i in inputs { let res = TeraParser :: parse (Rule :: template , i) ; println ! ("{:?}" , i) ; assert ! (res . is_err ()) ; } }
    };
}

lex_requires_whitespace_between_things!()