use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [doc = " Represents a dynamically loaded library containing procedural macros."] pub struct MacroDylib { path : AbsPathBuf , }
}