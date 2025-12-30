// Generated macro for test_const_fmt_int (function)
macro_rules! Depcrate_helperstest_const_fmt_int {
() => {
// Module: crate::helpers
// Provides: {"test_const_fmt_int"}
// Dependencies: {}
# [test] fn test_const_fmt_int () { assert_eq ! (* b"123" , const_fmt_int ::< 0 , 3 > (* b"" , 123)) ; assert_eq ! (* b"123   " , const_fmt_int ::< 0 , 6 > (* b"" , 123)) ; assert_eq ! (* b"abc123" , const_fmt_int ::< 3 , 6 > (* b"abc" , 123)) ; }
};
}
