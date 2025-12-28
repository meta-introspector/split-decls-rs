use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Deserialize)] pub struct AddItemDetails { pub target_file : PathBuf , pub item_code : String , }