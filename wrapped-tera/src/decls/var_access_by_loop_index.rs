macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! var_access_by_loop_index {
    () => {
        deps!();
        # [test] fn var_access_by_loop_index () { let context = Context :: new () ; let res = Tera :: one_off (r#"
{% set ics = ["fa-rocket","fa-paper-plane","fa-diamond","fa-signal"] %}
{% for a in ics %}
{{ ics[loop.index0] }}
{% endfor %}
    "# , & context , true ,) ; assert ! (res . is_ok ()) ; }
    };
}

var_access_by_loop_index!();