// Generated macro for check_runtime_no_duplicate_dependencies (function)
macro_rules! Depcrate_depscheck_runtime_no_duplicate_dependencies {
() => {
// Module: crate::deps
// Provides: {"check_runtime_no_duplicate_dependencies"}
// Dependencies: {}
fn check_runtime_no_duplicate_dependencies (metadata : & Metadata , bad : & mut bool) { let mut seen_pkgs = HashSet :: new () ; for pkg in & metadata . packages { if pkg . source . is_none () { continue ; } if pkg . name . to_string () != "wasi" && ! seen_pkgs . insert (& * pkg . name) { tidy_error ! (bad , "duplicate package `{}` is not allowed for the standard library" , pkg . name) ; } } }
};
}
