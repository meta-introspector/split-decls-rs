// Generated macro for has_missing_submodule (function)
macro_rules! Depcrate_depshas_missing_submodule {
() => {
// Module: crate::deps
// Provides: {"has_missing_submodule"}
// Dependencies: {}
# [doc = " Used to skip a check if a submodule is not checked out, and not in a CI environment."] # [doc = ""] # [doc = " This helps prevent enforcing developers to fetch submodules for tidy."] pub fn has_missing_submodule (root : & Path , submodules : & [& str]) -> bool { ! CiEnv :: is_ci () && submodules . iter () . any (| submodule | { let path = root . join (submodule) ; ! path . exists () || read_dir (path) . unwrap () . next () . is_none () }) }
};
}
