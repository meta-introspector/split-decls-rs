use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct IncludeVisitor { includes : Vec < PathBuf > , current_file_path : PathBuf , }
}