// Generated macro for fix_module_path (function)
macro_rules! Depcrate_fork_testfix_module_path {
() => {
// Module: crate::fork_test
// Provides: {"fix_module_path"}
// Dependencies: {}
# [doc = " Transform a string representing a qualified path as generated via"] # [doc = " `module_path!()` into a qualified path as expected by the standard Rust"] # [doc = " test harness."] pub fn fix_module_path (path : & str) -> & str { path . find ("::") . map (| ix | & path [ix + 2 ..]) . unwrap_or (path) }
};
}
