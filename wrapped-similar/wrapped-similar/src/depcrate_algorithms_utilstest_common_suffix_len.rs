// Generated macro for test_common_suffix_len (function)
macro_rules! Depcrate_algorithms_utilstest_common_suffix_len {
() => {
// Module: crate::algorithms::utils
// Provides: {"test_common_suffix_len"}
// Dependencies: {}
# [test] fn test_common_suffix_len () { assert_eq ! (common_suffix_len ("" . as_bytes () , 0 .. 0 , "" . as_bytes () , 0 .. 0) , 0) ; assert_eq ! (common_suffix_len ("1234" . as_bytes () , 0 .. 4 , "X0001234" . as_bytes () , 0 .. 8) , 4) ; assert_eq ! (common_suffix_len ("1234" . as_bytes () , 0 .. 4 , "Xxxx" . as_bytes () , 0 .. 4) , 0) ; assert_eq ! (common_suffix_len ("1234" . as_bytes () , 2 .. 4 , "01234" . as_bytes () , 2 .. 5) , 2) ; }
};
}
