// Generated macro for rustc_is_msrv (function)
macro_rules! Depcrate_utilsrustc_is_msrv {
() => {
// Module: crate::utils
// Provides: {"rustc_is_msrv"}
// Dependencies: {}
pub fn rustc_is_msrv () -> bool { const MSRV : & str = "1.76" ; let out = run_capturing_stdout (Command :: new ("rustc") . args (["-V"])) . unwrap () ; out . contains (MSRV) }
};
}
