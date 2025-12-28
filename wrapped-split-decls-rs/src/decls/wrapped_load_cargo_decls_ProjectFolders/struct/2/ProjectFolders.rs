use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Default)] pub struct ProjectFolders { pub load : Vec < vfs :: loader :: Entry > , pub watch : Vec < usize > , pub source_root_config : SourceRootConfig , }
}