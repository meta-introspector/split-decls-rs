use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: symlink_supported");
# [cfg (not (windows))] pub fn symlink_supported () -> bool { true }
}