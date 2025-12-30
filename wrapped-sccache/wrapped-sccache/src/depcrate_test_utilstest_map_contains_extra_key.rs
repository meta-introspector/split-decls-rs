// Generated macro for test_map_contains_extra_key (function)
macro_rules! Depcrate_test_utilstest_map_contains_extra_key {
() => {
// Module: crate::test::utils
// Provides: {"test_map_contains_extra_key"}
// Dependencies: {}
# [test] # [should_panic] fn test_map_contains_extra_key () { let mut m = HashMap :: new () ; m . insert ("a" , 1) ; m . insert ("b" , 2) ; assert_map_contains ! (m , ("a" , 1)) ; }
};
}
