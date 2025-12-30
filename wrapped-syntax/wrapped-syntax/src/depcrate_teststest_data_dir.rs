// Generated macro for test_data_dir (function)
macro_rules! Depcrate_teststest_data_dir {
() => {
// Module: crate::tests
// Provides: {"test_data_dir"}
// Dependencies: {}
fn test_data_dir () -> PathBuf { project_root () . into_std_path_buf () . join ("crates/syntax/test_data") }
};
}
