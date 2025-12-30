// Generated macro for rerun_if_git_changes (function)
macro_rules! Depcratererun_if_git_changes {
() => {
// Module: crate
// Provides: {"rerun_if_git_changes"}
// Dependencies: {}
# [must_use] pub fn rerun_if_git_changes () -> Option < () > { let git_head_file = PathBuf :: from (get_output ("git" , & ["rev-parse" , "--git-path" , "HEAD"]) ?) ; if git_head_file . exists () { println ! ("cargo::rerun-if-changed={}" , git_head_file . display ()) ; } let git_head_ref = get_output ("git" , & ["symbolic-ref" , "-q" , "HEAD"]) ? ; let git_head_ref_file = PathBuf :: from (get_output ("git" , & ["rev-parse" , "--git-path" , & git_head_ref]) ?) ; if git_head_ref_file . exists () { println ! ("cargo::rerun-if-changed={}" , git_head_ref_file . display ()) ; } Some (()) }
};
}
