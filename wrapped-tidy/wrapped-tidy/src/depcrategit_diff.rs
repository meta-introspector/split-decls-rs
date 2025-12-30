// Generated macro for git_diff (function)
macro_rules! Depcrategit_diff {
() => {
// Module: crate
// Provides: {"git_diff"}
// Dependencies: {}
pub fn git_diff < S : AsRef < OsStr > > (base_commit : & str , extra_arg : S) -> Option < String > { let output = Command :: new ("git") . arg ("diff") . arg (base_commit) . arg (extra_arg) . output () . ok () ? ; Some (String :: from_utf8_lossy (& output . stdout) . into ()) }
};
}
