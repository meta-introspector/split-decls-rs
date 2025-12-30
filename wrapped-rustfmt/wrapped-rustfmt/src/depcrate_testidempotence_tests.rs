// Generated macro for idempotence_tests (function)
macro_rules! Depcrate_testidempotence_tests {
() => {
// Module: crate::test
// Provides: {"idempotence_tests"}
// Dependencies: {}
# [nightly_only_test] # [test] fn idempotence_tests () { init_log () ; run_test_with (& TestSetting :: default () , | | { let files = get_test_files (Path :: new ("tests/target") , true) ; let (_reports , count , fails) = check_files (files , & None) ; println ! ("Ran {} idempotent tests." , count) ; assert_eq ! (fails , 0 , "{} idempotent tests failed" , fails) ; assert ! (count >= 400 , "Expected a minimum of {} idempotent tests to be executed" , 400) }) ; }
};
}
