use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclfn! {
println!("🔧 Calling function: get_cargo_tree_data");
pub fn get_cargo_tree_data () -> & 'static [CrateInfo] { & CARGO_TREE_DATA }
}