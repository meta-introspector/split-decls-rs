// Generated macro for default_filter_works_in_condition (function)
macro_rules! Depcrate_renderer_tests_basicdefault_filter_works_in_condition {
() => {
// Module: crate::renderer::tests::basic
// Provides: {"default_filter_works_in_condition"}
// Dependencies: {}
# [test] fn default_filter_works_in_condition () { let mut tera = Tera :: default () ; tera . add_raw_template ("test.html" , r#"{% if frobnicate|default(value=True) %}here{% endif %}"#) . unwrap () ; let res = tera . render ("test.html" , & Context :: new ()) ; assert_eq ! (res . unwrap () , "here") ; }
};
}
