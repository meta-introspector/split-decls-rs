use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// `trim_left_matches` has been deprecated in favor of `trim_start_matches`.
/// This helper silences the warning, as we need to continue using
/// `trim_left_matches` for rust 1.15 support.
#[allow(deprecated)]
fn trim_start_matches(s: &str, c: char) -> &str {
    s.trim_left_matches(c)
}
