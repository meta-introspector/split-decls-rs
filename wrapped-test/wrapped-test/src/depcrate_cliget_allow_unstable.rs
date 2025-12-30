// Generated macro for get_allow_unstable (function)
macro_rules! Depcrate_cliget_allow_unstable {
() => {
// Module: crate::cli
// Provides: {"get_allow_unstable"}
// Dependencies: {}
fn get_allow_unstable (matches : & getopts :: Matches) -> OptPartRes < bool > { let mut allow_unstable = false ; if let Some (opt) = matches . opt_str ("Z") { if ! is_nightly () { return Err ("the option `Z` is only accepted on the nightly compiler" . into ()) ; } match & * opt { "unstable-options" => { allow_unstable = true ; } _ => { return Err ("Unrecognized option to `Z`" . into ()) ; } } } ; Ok (allow_unstable) }
};
}
