use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Deserialize)] pub struct AddUseDetails { pub target_file : PathBuf , pub path : String , pub position : Option < AddUsePosition > , }