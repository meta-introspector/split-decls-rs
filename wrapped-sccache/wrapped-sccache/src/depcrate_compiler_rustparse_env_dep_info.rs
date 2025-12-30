// Generated macro for parse_env_dep_info (function)
macro_rules! Depcrate_compiler_rustparse_env_dep_info {
() => {
// Module: crate::compiler::rust
// Provides: {"parse_env_dep_info"}
// Dependencies: {}
fn parse_env_dep_info (dep_info : & str) -> Vec < (OsString , OsString) > { let mut env_deps = Vec :: new () ; for line in dep_info . lines () { if let Some (env_dep) = line . strip_prefix ("# env-dep:") { let mut split = env_dep . splitn (2 , '=') ; match (split . next () , split . next ()) { (Some (var) , Some (val)) => env_deps . push ((var . into () , val . into ())) , _ => env_deps . push ((env_dep . into () , "" . into ())) , } } } env_deps }
};
}
