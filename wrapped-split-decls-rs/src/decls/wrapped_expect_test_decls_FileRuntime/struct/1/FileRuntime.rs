use serde::{Deserialize, Serialize};
use std::collections::HashMap;

struct FileRuntime { path : PathBuf , original_text : String , patchwork : Patchwork , }