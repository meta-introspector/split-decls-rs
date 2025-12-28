macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! var_access_by_loop_index_with_set {
    () => {
        deps!();
        # [test] fn var_access_by_loop_index_with_set () { let context = Context :: new () ; let res = Tera :: one_off (r#"
{% set ics = ["fa-rocket","fa-paper-plane","fa-diamond","fa-signal"] %}
{% for a in ics %}
    {% set i = loop.index - 1 %}
    {{ ics[i] }}
{% endfor %}
    "# , & context , true ,) ; assert ! (res . is_ok ()) ; }
    };
}

var_access_by_loop_index_with_set!();