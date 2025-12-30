// Generated macro for test_ratio (function)
macro_rules! Depcrate_texttest_ratio {
() => {
// Module: crate::text
// Provides: {"test_ratio"}
// Dependencies: {}
# [test] fn test_ratio () { let diff = TextDiff :: from_chars ("abcd" , "bcde") ; assert_eq ! (diff . ratio () , 0.75) ; let diff = TextDiff :: from_chars ("" , "") ; assert_eq ! (diff . ratio () , 1.0) ; }
};
}
