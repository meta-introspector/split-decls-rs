// Generated macro for filter (function)
macro_rules! Depcrate_tests_revision_unpaired_stdout_stderrfilter {
() => {
// Module: crate::tests_revision_unpaired_stdout_stderr
// Provides: {"filter"}
// Dependencies: {}
fn filter (path : & Path) -> bool { filter_dirs (path) || (path . file_name () . is_some_and (| name | name == "auxiliary")) }
};
}
