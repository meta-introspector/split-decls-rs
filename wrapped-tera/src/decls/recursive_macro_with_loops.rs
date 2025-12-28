macro_rules! deps {
    () => {
        Tera!();
        NestedObject!();
        Context!();
    };
}

macro_rules! recursive_macro_with_loops {
    () => {
        deps!();
        # [test] fn recursive_macro_with_loops () { let parent = NestedObject { label : "Parent" . to_string () , parent : None , numbers : vec ! [1 , 2 , 3] } ; let child = NestedObject { label : "Child" . to_string () , parent : Some (Box :: new (parent)) , numbers : vec ! [1 , 2 , 3] , } ; let mut context = Context :: new () ; context . insert ("objects" , & vec ! [child]) ; let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("macros.html" , r#"
{% macro label_for(obj, sep) -%}
  {%- if obj.parent -%}
    {{ self::label_for(obj=obj.parent, sep=sep) }}{{sep}}
  {%- endif -%}
  {{obj.label}}
  {%- for i in obj.numbers -%}{{ i }}{%- endfor -%}
{%- endmacro label_for %}
            "# ,) , ("recursive" , r#"
{%- import "macros.html" as macros -%}
{%- for obj in objects -%}
    {{ macros::label_for(obj=obj, sep="|") }}
{%- endfor -%}
"# ,) ,]) . unwrap () ; let result = tera . render ("recursive" , & context) ; assert_eq ! (result . unwrap () , "Parent123|Child123" . to_string ()) ; }
    };
}

recursive_macro_with_loops!();