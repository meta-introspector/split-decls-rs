macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! can_remove_whitespace_inheritance {
    () => {
        deps!();
        # [test] fn can_remove_whitespace_inheritance () { let mut context = Context :: new () ; context . insert ("numbers" , & vec ! [1 , 2 , 3]) ; let inputs = vec ! [(r#"{%- extends "base" -%} {% block content %}{{super()}}{% endblock %}"# , " Hey! ") , (r#"{%- extends "base" -%} {% block content -%}{{super()}}{%- endblock %}"# , " Hey! ") , (r#"{%- extends "base" %} {%- block content -%}{{super()}}{%- endblock -%} "# , " Hey! ") ,] ; for (input , expected) in inputs { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("base" , "{% block content %} Hey! {% endblock %}") , ("tpl" , input) ,]) . unwrap () ; assert_eq ! (tera . render ("tpl" , & context) . unwrap () , expected) ; } }
    };
}

can_remove_whitespace_inheritance!();