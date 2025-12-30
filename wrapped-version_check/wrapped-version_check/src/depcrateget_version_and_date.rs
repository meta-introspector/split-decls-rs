// Generated macro for get_version_and_date (function)
macro_rules! Depcrateget_version_and_date {
() => {
// Module: crate
// Provides: {"get_version_and_date"}
// Dependencies: {}
# [doc = " Returns (version, date) as available from `rustc --version`."] fn get_version_and_date () -> Option < (Option < String > , Option < String >) > { let rustc = env :: var ("RUSTC") . unwrap_or_else (| _ | "rustc" . to_string ()) ; Command :: new (rustc) . arg ("--verbose") . arg ("--version") . output () . ok () . and_then (| output | String :: from_utf8 (output . stdout) . ok ()) . map (| s | version_and_date_from_rustc_verbose_version (& s)) }
};
}
