// Generated macro for coverage_tests (function)
macro_rules! Depcrate_testcoverage_tests {
() => {
// Module: crate::test
// Provides: {"coverage_tests"}
// Dependencies: {}
# [test] fn coverage_tests () { init_log () ; let files = get_test_files (Path :: new ("tests/coverage/source") , true) ; let (_reports , count , fails) = check_files (files , & None) ; println ! ("Ran {count} tests in coverage mode.") ; assert_eq ! (fails , 0 , "{fails} tests failed") ; }
};
}
