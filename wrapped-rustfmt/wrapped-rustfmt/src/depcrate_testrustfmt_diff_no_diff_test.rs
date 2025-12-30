// Generated macro for rustfmt_diff_no_diff_test (function)
macro_rules! Depcrate_testrustfmt_diff_no_diff_test {
() => {
// Module: crate::test
// Provides: {"rustfmt_diff_no_diff_test"}
// Dependencies: {}
# [test] fn rustfmt_diff_no_diff_test () { init_log () ; let diff = make_diff ("a\nb\nc\nd" , "a\nb\nc\nd" , 3) ; assert_eq ! (diff , vec ! []) ; }
};
}
