use serde::{Deserialize, Serialize};
use std::collections::HashMap;

# [doc = " Represents a dynamically loaded library containing procedural macros."] pub struct MacroDylib { path : AbsPathBuf , }