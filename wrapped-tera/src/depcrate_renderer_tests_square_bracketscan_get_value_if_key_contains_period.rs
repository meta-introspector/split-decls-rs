// Generated macro for can_get_value_if_key_contains_period (function)
macro_rules! Depcrate_renderer_tests_square_bracketscan_get_value_if_key_contains_period {
() => {
// Module: crate::renderer::tests::square_brackets
// Provides: {"can_get_value_if_key_contains_period"}
// Dependencies: {}
# [test] fn can_get_value_if_key_contains_period () { let mut context = Context :: new () ; context . insert ("name" , "Mt. Robson Provincial Park") ; let mut map = HashMap :: new () ; map . insert ("Mt. Robson Provincial Park" . to_string () , "hello" . to_string ()) ; context . insert ("tag_info" , & map) ; let res = Tera :: one_off (r#"{{ tag_info[name] }}"# , & context , true) ; assert ! (res . is_ok ()) ; let res = res . unwrap () ; assert_eq ! (res , "hello") ; }
};
}
