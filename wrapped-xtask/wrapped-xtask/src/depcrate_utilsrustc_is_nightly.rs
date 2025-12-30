// Generated macro for rustc_is_nightly (function)
macro_rules! Depcrate_utilsrustc_is_nightly {
() => {
// Module: crate::utils
// Provides: {"rustc_is_nightly"}
// Dependencies: {}
pub fn rustc_is_nightly () -> bool { let out = run_capturing_stdout (Command :: new ("rustc") . args (["-V"])) . unwrap () ; out . contains ("nightly") }
};
}
