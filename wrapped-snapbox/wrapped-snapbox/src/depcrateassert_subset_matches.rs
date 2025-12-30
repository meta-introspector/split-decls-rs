// Generated macro for assert_subset_matches (function)
macro_rules! Depcrateassert_subset_matches {
() => {
// Module: crate
// Provides: {"assert_subset_matches"}
// Dependencies: {}
# [doc = " Check if a path matches the pattern of another path, recursively"] # [doc = ""] # [doc = " Pattern syntax:"] # [doc = " - `...` is a line-wildcard when on a line by itself"] # [doc = " - `[..]` is a character-wildcard when inside a line"] # [doc = " - `[EXE]` matches `.exe` on Windows"] # [doc = ""] # [doc = " Normalization:"] # [doc = " - Newlines"] # [doc = " - `\\` to `/`"] # [doc = ""] # [doc = " ```rust,no_run"] # [doc = " let output_root = \"...\";"] # [doc = " let expected_root = \"tests/snapshots/output.txt\";"] # [doc = " snapbox::assert_subset_matches(expected_root, output_root);"] # [doc = " ```"] # [cfg (feature = "dir")] # [track_caller] pub fn assert_subset_matches (pattern_root : impl Into < std :: path :: PathBuf > , actual_root : impl Into < std :: path :: PathBuf > ,) { Assert :: new () . action_env (assert :: DEFAULT_ACTION_ENV) . subset_matches (pattern_root , actual_root) ; }
};
}
