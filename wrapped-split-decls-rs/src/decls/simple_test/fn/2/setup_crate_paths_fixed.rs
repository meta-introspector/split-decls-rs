use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: setup_crate_paths_fixed");
fn setup_crate_paths_fixed (crate_path : & Path) -> Result < CratePaths > { setup_crate_paths (crate_path) }
}