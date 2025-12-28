use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
struct MacroVisitor { macros : Vec < MacroInfo > , file_path : PathBuf , }
}