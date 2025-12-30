// Generated macro for system_tests (function)
macro_rules! Depcrate_testsystem_tests {
() => {
// Module: crate::test
// Provides: {"system_tests"}
// Dependencies: {}
# [test] fn system_tests () { init_log () ; run_test_with (& TestSetting :: default () , | | { let files = get_test_files (Path :: new ("tests/source") , true) ; let (_reports , count , fails) = check_files (files , & None) ; println ! ("Ran {} system tests." , count) ; assert_eq ! (fails , 0 , "{} system tests failed" , fails) ; assert ! (count >= 300 , "Expected a minimum of {} system tests to be executed" , 300) }) ; }
};
}
