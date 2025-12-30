// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl CiInfo { pub fn new (bad : & mut bool) -> Self { let stage0 = parse_stage0_file () ; let Stage0Config { nightly_branch , git_merge_commit_email , .. } = stage0 . config ; let mut info = Self { nightly_branch , git_merge_commit_email , ci_env : CiEnv :: current () , base_commit : None , } ; let base_commit = match get_closest_upstream_commit (None , & info . git_config () , info . ci_env) { Ok (Some (commit)) => Some (commit) , Ok (None) => { info . error_if_in_ci ("no base commit found" , bad) ; None } Err (error) => { info . error_if_in_ci (& format ! ("failed to retrieve base commit: {error}") , bad) ; None } } ; info . base_commit = base_commit ; info } pub fn git_config (& self) -> GitConfig < '_ > { GitConfig { nightly_branch : & self . nightly_branch , git_merge_commit_email : & self . git_merge_commit_email , } } pub fn error_if_in_ci (& self , msg : & str , bad : & mut bool) { if self . ci_env . is_running_in_ci () { * bad = true ; eprintln ! ("tidy check error: {msg}") ; } else { eprintln ! ("tidy check warning: {msg}. Some checks will be skipped.") ; } } }
};
}
