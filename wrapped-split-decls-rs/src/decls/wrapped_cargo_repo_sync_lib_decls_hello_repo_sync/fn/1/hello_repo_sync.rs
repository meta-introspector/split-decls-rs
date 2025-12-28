use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
pub fn hello_repo_sync () -> String { "Hello from cargo-repo-sync-lib!" . to_string () }
}