macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! redefining_loop_value_doesnt_break_loop {
    () => {
        deps!();
        # [test] fn redefining_loop_value_doesnt_break_loop () { let mut tera = Tera :: default () ; tera . add_raw_template ("tpl" , r#"
{%- set string = "abcdefghdijklm" | split(pat="d") -%}
{% for i in string -%}
    {%- set j = i ~ "lol" ~ " " -%}
    {{ j }}
{%- endfor -%}
        "# ,) . unwrap () ; let context = Context :: new () ; let result = tera . render ("tpl" , & context) ; assert_eq ! (result . unwrap () , "abclol efghlol ijklmlol ") ; }
    };
}

redefining_loop_value_doesnt_break_loop!();