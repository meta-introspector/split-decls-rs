// Generated macro for get_typos_version (function)
macro_rules! Depcrateget_typos_version {
() => {
// Module: crate
// Provides: {"get_typos_version"}
// Dependencies: {}
fn get_typos_version (bin : & PathBuf) -> Option < Version > { if let Ok (output) = Command :: new (& bin) . arg ("--version") . output () && let Ok (s) = String :: from_utf8 (output . stdout) && let Some (version_str) = s . trim () . split_whitespace () . last () { Version :: parse (version_str) . ok () } else { None } }
};
}
