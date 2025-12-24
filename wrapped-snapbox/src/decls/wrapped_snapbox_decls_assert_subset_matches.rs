use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Check if a path matches the pattern of another path, recursively
///
/// Pattern syntax:
/// - `...` is a line-wildcard when on a line by itself
/// - `[..]` is a character-wildcard when inside a line
/// - `[EXE]` matches `.exe` on Windows
///
/// Normalization:
/// - Newlines
/// - `\` to `/`
///
/// ```rust,no_run
/// let output_root = "...";
/// let expected_root = "tests/snapshots/output.txt";
/// snapbox::assert_subset_matches(expected_root, output_root);
/// ```
#[cfg(feature = "dir")]
#[track_caller]
pub fn assert_subset_matches(
    pattern_root: impl Into<std::path::PathBuf>,
    actual_root: impl Into<std::path::PathBuf>,
) {
    Assert::new()
        .action_env(assert::DEFAULT_ACTION_ENV)
        .subset_matches(pattern_root, actual_root);
}
