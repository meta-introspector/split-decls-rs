use serde::{Deserialize, Serialize};
use std::collections::HashMap;
/// Returns `false` if slow tests should not run, otherwise returns `true` and
/// also creates a file at `./target/.slow_tests_cookie` which serves as a flag
/// that slow tests did run.
pub fn skip_slow_tests() -> bool {
    let should_skip = (std::env::var("CI").is_err()
        && std::env::var("RUN_SLOW_TESTS").is_err())
        || std::env::var("SKIP_SLOW_TESTS").is_ok();
    if should_skip {
        eprintln!("ignoring slow test");
    } else {
        let path = target_dir().join(".slow_tests_cookie");
        fs::write(path, ".").unwrap();
    }
    should_skip
}
