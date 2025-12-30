// Generated macro for impl_132 (impl)
macro_rules! Depcrateimpl_132 {
() => {
// Module: crate
// Provides: {"impl_132"}
// Dependencies: {}
impl CheckerError { # [doc = " Generate a CheckFailed error with an empty debug_step"] fn check_failed (step : u64 , msg : String) -> CheckerError { CheckerError :: CheckFailed { step , msg , debug_step : String :: new () , } } }
};
}
