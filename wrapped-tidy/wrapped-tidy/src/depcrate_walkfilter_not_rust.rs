// Generated macro for filter_not_rust (function)
macro_rules! Depcrate_walkfilter_not_rust {
() => {
// Module: crate::walk
// Provides: {"filter_not_rust"}
// Dependencies: {}
# [doc = " Filter for only files that end in `.rs`."] pub fn filter_not_rust (path : & Path) -> bool { path . extension () != Some (OsStr :: new ("rs")) && ! path . is_dir () }
};
}
