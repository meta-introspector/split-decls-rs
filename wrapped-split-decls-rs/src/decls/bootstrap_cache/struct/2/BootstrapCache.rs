use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Default , Serialize , Deserialize)] pub struct BootstrapCache { files : HashMap < PathBuf , FileCache > , git_tree_hash : Option < String > , }
}