// Generated macro for get_run_ignored (function)
macro_rules! Depcrate_cliget_run_ignored {
() => {
// Module: crate::cli
// Provides: {"get_run_ignored"}
// Dependencies: {}
fn get_run_ignored (matches : & getopts :: Matches , include_ignored : bool) -> OptPartRes < RunIgnored > { let run_ignored = match (include_ignored , matches . opt_present ("ignored")) { (true , true) => { return Err ("the options --include-ignored and --ignored are mutually exclusive" . into ()) ; } (true , false) => RunIgnored :: Yes , (false , true) => RunIgnored :: Only , (false , false) => RunIgnored :: No , } ; Ok (run_ignored) }
};
}
