use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [derive (Debug , Deserialize)] pub struct RemoveUseDetails { pub target_file : PathBuf , pub use_path : String , }