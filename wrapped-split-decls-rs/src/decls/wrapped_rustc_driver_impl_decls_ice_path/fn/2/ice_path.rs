use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: ice_path");
fn ice_path () -> & 'static Option < PathBuf > { ice_path_with_config (None) }
}