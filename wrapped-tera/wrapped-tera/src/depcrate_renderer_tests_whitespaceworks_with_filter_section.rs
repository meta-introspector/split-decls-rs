// Generated macro for works_with_filter_section (function)
macro_rules! Depcrate_renderer_tests_whitespaceworks_with_filter_section {
() => {
// Module: crate::renderer::tests::whitespace
// Provides: {"works_with_filter_section"}
// Dependencies: {}
# [test] fn works_with_filter_section () { let mut context = Context :: new () ; context . insert ("d" , "d") ; let input = r#"{% filter upper %}  {{ "c" }}   d{% endfilter %}"# ; let res = Tera :: one_off (input , & context , true) . unwrap () ; assert_eq ! (res , "  C   D") ; }
};
}
