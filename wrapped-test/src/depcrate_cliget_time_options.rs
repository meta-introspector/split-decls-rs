// Generated macro for get_time_options (function)
macro_rules! Depcrate_cliget_time_options {
() => {
// Module: crate::cli
// Provides: {"get_time_options"}
// Dependencies: {}
fn get_time_options (matches : & getopts :: Matches , allow_unstable : bool ,) -> OptPartRes < Option < TestTimeOptions > > { let report_time = unstable_optflag ! (matches , allow_unstable , "report-time") ; let ensure_test_time = unstable_optflag ! (matches , allow_unstable , "ensure-time") ; let options = if report_time || ensure_test_time { Some (TestTimeOptions :: new_from_env (ensure_test_time)) } else { None } ; Ok (options) }
};
}
