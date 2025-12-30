// Generated macro for TestTimeOptions (struct)
macro_rules! Depcrate_timeTestTimeOptions {
() => {
// Module: crate::time
// Provides: {"TestTimeOptions"}
// Dependencies: {}
# [doc = " Structure with parameters for calculating test execution time."] # [derive (Copy , Clone , Debug , Default , PartialEq , Eq)] pub struct TestTimeOptions { # [doc = " Denotes if the test critical execution time limit excess should be considered"] # [doc = " a test failure."] pub error_on_excess : bool , pub unit_threshold : TimeThreshold , pub integration_threshold : TimeThreshold , pub doctest_threshold : TimeThreshold , }
};
}
