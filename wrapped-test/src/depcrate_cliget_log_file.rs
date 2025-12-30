// Generated macro for get_log_file (function)
macro_rules! Depcrate_cliget_log_file {
() => {
// Module: crate::cli
// Provides: {"get_log_file"}
// Dependencies: {}
fn get_log_file (matches : & getopts :: Matches) -> OptPartRes < Option < PathBuf > > { let logfile = matches . opt_str ("logfile") . map (| s | PathBuf :: from (& s)) ; Ok (logfile) }
};
}
