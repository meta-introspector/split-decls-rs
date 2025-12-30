// Generated macro for get_default_test_timeout (function)
macro_rules! Depcrate_timeget_default_test_timeout {
() => {
// Module: crate::time
// Provides: {"get_default_test_timeout"}
// Dependencies: {}
# [doc = " Returns an `Instance` object denoting when the test should be considered"] # [doc = " timed out."] pub (crate) fn get_default_test_timeout () -> Instant { Instant :: now () + Duration :: from_secs (TEST_WARN_TIMEOUT_S) }
};
}
