// Generated macro for rust_files_in_dir (function)
macro_rules! Depcrate_testsrust_files_in_dir {
() => {
// Module: crate::tests
// Provides: {"rust_files_in_dir"}
// Dependencies: {}
# [doc = " Collects paths to all `.rs` files from `dir` in a sorted `Vec<PathBuf>`."] fn rust_files_in_dir (dir : & Path) -> Vec < PathBuf > { let mut acc = Vec :: new () ; for file in fs :: read_dir (dir) . unwrap () { let file = file . unwrap () ; let path = file . path () ; if path . extension () . unwrap_or_default () == "rs" { acc . push (path) ; } } acc . sort () ; acc }
};
}
