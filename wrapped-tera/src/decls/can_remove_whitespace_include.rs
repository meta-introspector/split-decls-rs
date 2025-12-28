macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! can_remove_whitespace_include {
    () => {
        deps!();
        # [test] fn can_remove_whitespace_include () { let mut context = Context :: new () ; context . insert ("numbers" , & vec ! [1 , 2 , 3]) ; let inputs = vec ! [(r#"Hi {%- include "include" -%} "# , "HiIncluded") , (r#"Hi {% include "include" -%} "# , "Hi Included") , (r#"Hi {% include "include" %} "# , "Hi Included ") ,] ; for (input , expected) in inputs { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("include" , "Included") , ("tpl" , input)]) . unwrap () ; assert_eq ! (tera . render ("tpl" , & context) . unwrap () , expected) ; } }
    };
}

can_remove_whitespace_include!();