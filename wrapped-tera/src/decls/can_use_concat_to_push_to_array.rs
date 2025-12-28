macro_rules! deps {
    () => {
        Tera!();
        Context!();
    };
}

macro_rules! can_use_concat_to_push_to_array {
    () => {
        deps!();
        # [test] fn can_use_concat_to_push_to_array () { let mut tera = Tera :: default () ; tera . add_raw_template ("tpl" , r#"
{%- set ids = [] -%}
{% for i in range(end=5) -%}
{%- set_global ids = ids | concat(with=i) -%}
{%- endfor -%}
{{ids}}"# ,) . unwrap () ; let context = Context :: new () ; let result = tera . render ("tpl" , & context) ; assert_eq ! (result . unwrap () , "[0, 1, 2, 3, 4]") ; }
    };
}

can_use_concat_to_push_to_array!()