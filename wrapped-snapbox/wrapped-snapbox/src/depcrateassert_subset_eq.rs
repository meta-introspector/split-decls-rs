// Generated macro for assert_subset_eq (function)
macro_rules! Depcrateassert_subset_eq {
() => {
// Module: crate
// Provides: {"assert_subset_eq"}
// Dependencies: {}
# [doc = " Check if a path matches the content of another path, recursively"] # [doc = ""] # [doc = " When the content is text, newlines are normalized."] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " let output_root = \"...\";"] # [doc = " let expected_root = \"tests/snapshots/output.txt\";"] # [doc = " snapbox::assert_subset_eq(expected_root, output_root);"] # [doc = " ```"] # [cfg (feature = "dir")] # [track_caller] pub fn assert_subset_eq (expected_root : impl Into < std :: path :: PathBuf > , actual_root : impl Into < std :: path :: PathBuf > ,) { Assert :: new () . action_env (assert :: DEFAULT_ACTION_ENV) . subset_eq (expected_root , actual_root) ; }
};
}
