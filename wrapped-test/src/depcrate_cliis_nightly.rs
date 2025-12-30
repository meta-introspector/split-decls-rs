// Generated macro for is_nightly (function)
macro_rules! Depcrate_cliis_nightly {
() => {
// Module: crate::cli
// Provides: {"is_nightly"}
// Dependencies: {}
fn is_nightly () -> bool { let disable_unstable_features = option_env ! ("CFG_DISABLE_UNSTABLE_FEATURES") . map (| s | s != "0") . unwrap_or (false) ; let bootstrap = env :: var ("RUSTC_BOOTSTRAP") . is_ok () ; bootstrap || ! disable_unstable_features }
};
}
