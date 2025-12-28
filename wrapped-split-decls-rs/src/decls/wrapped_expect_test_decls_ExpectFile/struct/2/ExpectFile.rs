use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Self-updating file."] # [derive (Debug)] pub struct ExpectFile { # [doc (hidden)] pub path : PathBuf , # [doc (hidden)] pub position : & 'static str , }
}