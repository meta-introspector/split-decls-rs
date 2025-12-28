macro_rules! deps {
    () => {
        TeraParser!();
    };
}

macro_rules! lex_template {
    () => {
        deps!();
        # [test] fn lex_template () { assert ! (TeraParser :: parse (Rule :: template , "{# Greeter template #}
            Hello {% if i18n %}世界{% else %}world{% endif %}
            {% for country in countries %}
                {{ loop.index }}.{{ country }}
            {% endfor %}" ,) . is_ok ()) ; }
    };
}

lex_template!()