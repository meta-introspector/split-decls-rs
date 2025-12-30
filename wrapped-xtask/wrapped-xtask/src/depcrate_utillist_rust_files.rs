// Generated macro for list_rust_files (function)
macro_rules! Depcrate_utillist_rust_files {
() => {
// Module: crate::util
// Provides: {"list_rust_files"}
// Dependencies: {}
pub (crate) fn list_rust_files (dir : & Path) -> Vec < PathBuf > { let mut res = list_files (dir) ; res . retain (| it | { it . file_name () . unwrap_or_default () . to_str () . unwrap_or_default () . ends_with (".rs") }) ; res }
};
}
