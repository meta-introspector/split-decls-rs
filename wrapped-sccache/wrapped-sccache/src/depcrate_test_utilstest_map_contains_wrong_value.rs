// Generated macro for test_map_contains_wrong_value (function)
macro_rules! Depcrate_test_utilstest_map_contains_wrong_value {
() => {
// Module: crate::test::utils
// Provides: {"test_map_contains_wrong_value"}
// Dependencies: {}
# [test] # [should_panic] fn test_map_contains_wrong_value () { let mut m = HashMap :: new () ; m . insert ("a" , 1) ; m . insert ("b" , 3) ; assert_map_contains ! (m , ("a" , 1) , ("b" , 2)) ; }
};
}
