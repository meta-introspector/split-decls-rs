// Generated macro for get_commit_hash (function)
macro_rules! Depcrateget_commit_hash {
() => {
// Module: crate
// Provides: {"get_commit_hash"}
// Dependencies: {}
# [must_use] pub fn get_commit_hash () -> Option < String > { let mut stdout = get_output ("git" , & ["rev-parse" , "HEAD"]) ? ; stdout . truncate (10) ; Some (stdout) }
};
}
