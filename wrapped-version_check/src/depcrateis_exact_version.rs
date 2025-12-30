// Generated macro for is_exact_version (function)
macro_rules! Depcrateis_exact_version {
() => {
// Module: crate
// Provides: {"is_exact_version"}
// Dependencies: {}
# [doc = " Checks that the running or installed `rustc` is **exactly** some version."] # [doc = ""] # [doc = " The format of `version` is a semantic version: `1.3.0`, `1.15.0-beta`,"] # [doc = " `1.14.0`, `1.16.0-nightly`, etc."] # [doc = ""] # [doc = " If the version cannot be retrieved or parsed, or if `version` could not be"] # [doc = " parsed, returns `None`. Otherwise returns `true` if the installed `rustc` is"] # [doc = " exactly `version` and `false` otherwise."] pub fn is_exact_version (version : & str) -> Option < bool > { match (Version :: read () , Version :: parse (version)) { (Some (rustc_ver) , Some (version)) => Some (rustc_ver == version) , _ => None , } }
};
}
