// Generated macro for collect_rust_files (function)
macro_rules! Depcrate_testscollect_rust_files {
() => {
// Module: crate::tests
// Provides: {"collect_rust_files"}
// Dependencies: {}
# [doc = " Collects all `.rs` files from `dir` subdirectories defined by `paths`."] fn collect_rust_files (root_dir : & Path , paths : & [& str]) -> Vec < (PathBuf , String) > { paths . iter () . flat_map (| path | { let path = root_dir . to_owned () . join (path) ; rust_files_in_dir (& path) . into_iter () }) . map (| path | { let text = read_text (& path) ; (path , text) }) . collect () }
};
}
