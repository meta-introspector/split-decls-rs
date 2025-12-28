use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [salsa_macros :: input (debug)] pub struct FileSourceRootInput { pub source_root_id : SourceRootId , }
}