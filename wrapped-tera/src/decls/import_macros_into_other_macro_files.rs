macro_rules! deps {
    () => {
        Context!();
        Tera!();
    };
}

macro_rules! import_macros_into_other_macro_files {
    () => {
        deps!();
        # [test] fn import_macros_into_other_macro_files () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("submacros" , "{% macro test() %}Success!{% endmacro %}") , ("macros" , r#"{% import "submacros" as sub %}{% macro test() %}{{ sub::test() }}{% endmacro %}"# ,) , ("index" , r#"{% import "macros" as macros %}{{ macros::test() }}"#) ,]) . unwrap () ; let result = tera . render ("index" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "Success!" . to_string ()) ; }
    };
}

import_macros_into_other_macro_files!()