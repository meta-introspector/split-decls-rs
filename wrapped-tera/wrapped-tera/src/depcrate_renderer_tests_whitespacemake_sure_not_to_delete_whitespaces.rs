// Generated macro for make_sure_not_to_delete_whitespaces (function)
macro_rules! Depcrate_renderer_tests_whitespacemake_sure_not_to_delete_whitespaces {
() => {
// Module: crate::renderer::tests::whitespace
// Provides: {"make_sure_not_to_delete_whitespaces"}
// Dependencies: {}
# [test] fn make_sure_not_to_delete_whitespaces () { let mut context = Context :: new () ; context . insert ("d" , "d") ; let input = r#"{% raw %}    yaml_test:     {% endraw %}"# ; let res = Tera :: one_off (input , & context , true) . unwrap () ; assert_eq ! (res , "    yaml_test:     ") ; }
};
}
