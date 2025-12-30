// Generated macro for test_error_on_exceed (function)
macro_rules! Depcrate_stats_teststest_error_on_exceed {
() => {
// Module: crate::stats::tests
// Provides: {"test_error_on_exceed"}
// Dependencies: {}
# [test] fn test_error_on_exceed () { let types = [TestType :: UnitTest , TestType :: IntegrationTest , TestType :: DocTest] ; for test_type in types . iter () { let result = time_test_failure_template (* test_type) ; assert_eq ! (result , TestResult :: TrTimedFail) ; } let result = time_test_failure_template (TestType :: Unknown) ; assert_eq ! (result , TestResult :: TrOk) ; }
};
}
