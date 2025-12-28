use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn get_cargo_tree_data () -> & 'static [CrateInfo] { & CARGO_TREE_DATA }