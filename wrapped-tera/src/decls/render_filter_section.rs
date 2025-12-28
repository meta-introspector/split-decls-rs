macro_rules! deps {
    () => {
        Context!();
    };
}

macro_rules! render_filter_section {
    () => {
        deps!();
        # [test] fn render_filter_section () { let inputs = vec ! [("{% filter upper %}Hello{% endfilter %}" , "HELLO") , ("{% filter upper %}Hello{% if true %} world{% endif %}{% endfilter %}" , "HELLO WORLD") , ("{% filter upper %}Hello {% for i in range(end=3) %}i{% endfor %}{% endfilter %}" , "HELLO III") , ("{% filter upper %}Hello {% for i in range(end=3) %}{% if i == 1 %}{% break %} {% endif %}i{% endfor %}{% endfilter %}" , "HELLO I" ,) , ("{% filter title %}Hello {% if true %}{{ 'world' | upper | safe }}{% endif %}{% endfilter %}" , "Hello World") , ("{% filter safe %}{% filter upper %}<Hello>{% endfilter %}{% endfilter%}" , "<HELLO>")] ; let context = Context :: new () ; for (input , expected) in inputs { println ! ("{:?} -> {:?}" , input , expected) ; assert_eq ! (render_template (input , & context) . unwrap () , expected) ; } }
    };
}

render_filter_section!();