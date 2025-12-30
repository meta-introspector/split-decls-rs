// Generated macro for test_common_prefix_len (function)
macro_rules! Depcrate_algorithms_utilstest_common_prefix_len {
() => {
// Module: crate::algorithms::utils
// Provides: {"test_common_prefix_len"}
// Dependencies: {}
# [test] fn test_common_prefix_len () { assert_eq ! (common_prefix_len ("" . as_bytes () , 0 .. 0 , "" . as_bytes () , 0 .. 0) , 0) ; assert_eq ! (common_prefix_len ("foobarbaz" . as_bytes () , 0 .. 9 , "foobarblah" . as_bytes () , 0 .. 10) , 7) ; assert_eq ! (common_prefix_len ("foobarbaz" . as_bytes () , 0 .. 9 , "blablabla" . as_bytes () , 0 .. 9) , 0) ; assert_eq ! (common_prefix_len ("foobarbaz" . as_bytes () , 3 .. 9 , "foobarblah" . as_bytes () , 3 .. 10) , 4) ; }
};
}
