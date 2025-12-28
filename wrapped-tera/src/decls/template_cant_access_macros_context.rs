macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! template_cant_access_macros_context {
    () => {
        deps!();
        # [test] fn template_cant_access_macros_context () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("parent" , r#"{% import "macros" as macros %}{{ macros::empty() }}{{ quote | default(value="I'd rather have roses on my table than diamonds on my neck.") }}"#) , ("macros" , r#"{% macro empty() %}{% set_global quote = "This should not reachable from the calling template!" %}{% endmacro empty %}"#)]) . unwrap () ; let result = tera . render ("parent" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "I'd rather have roses on my table than diamonds on my neck.") ; }
    };
}

template_cant_access_macros_context!()