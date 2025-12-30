// Generated macro for import_macros_into_other_macro_files (function)
macro_rules! Depcrate_renderer_tests_macrosimport_macros_into_other_macro_files {
() => {
// Module: crate::renderer::tests::macros
// Provides: {"import_macros_into_other_macro_files"}
// Dependencies: {}
# [test] fn import_macros_into_other_macro_files () { let mut tera = Tera :: default () ; tera . add_raw_templates (vec ! [("submacros" , "{% macro test() %}Success!{% endmacro %}") , ("macros" , r#"{% import "submacros" as sub %}{% macro test() %}{{ sub::test() }}{% endmacro %}"# ,) , ("index" , r#"{% import "macros" as macros %}{{ macros::test() }}"#) ,]) . unwrap () ; let result = tera . render ("index" , & Context :: new ()) ; assert_eq ! (result . unwrap () , "Success!" . to_string ()) ; }
};
}
