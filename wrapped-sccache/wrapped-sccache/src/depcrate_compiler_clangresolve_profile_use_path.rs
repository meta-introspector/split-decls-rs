// Generated macro for resolve_profile_use_path (function)
macro_rules! Depcrate_compiler_clangresolve_profile_use_path {
() => {
// Module: crate::compiler::clang
// Provides: {"resolve_profile_use_path"}
// Dependencies: {}
pub (crate) fn resolve_profile_use_path (arg : & Path , cwd : & Path) -> PathBuf { let mut path = cwd . join (arg) ; assert ! (! arg . as_os_str () . is_empty () || path == cwd) ; if path . is_dir () { path . push ("default.profdata") ; } path }
};
}
