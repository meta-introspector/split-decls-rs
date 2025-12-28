use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: hello_repo_sync");
pub fn hello_repo_sync () -> String { "Hello from cargo-repo-sync-lib!" . to_string () }
}