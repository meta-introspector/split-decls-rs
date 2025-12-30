// Generated macro for rustfmt_diff_make_diff_tests (function)
macro_rules! Depcrate_testrustfmt_diff_make_diff_tests {
() => {
// Module: crate::test
// Provides: {"rustfmt_diff_make_diff_tests"}
// Dependencies: {}
# [test] fn rustfmt_diff_make_diff_tests () { init_log () ; let diff = make_diff ("a\nb\nc\nd" , "a\ne\nc\nd" , 3) ; assert_eq ! (diff , vec ! [Mismatch { line_number : 1 , line_number_orig : 1 , lines : vec ! [DiffLine :: Context ("a" . into ()) , DiffLine :: Resulting ("b" . into ()) , DiffLine :: Expected ("e" . into ()) , DiffLine :: Context ("c" . into ()) , DiffLine :: Context ("d" . into ()) ,] , }]) ; }
};
}
