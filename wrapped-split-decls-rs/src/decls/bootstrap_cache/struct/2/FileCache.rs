use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclstruct! {
# [derive (Debug , Clone , Serialize , Deserialize)] pub struct FileCache { pub content : String , pub git_hash : Option < String > , pub last_modified : u64 , }
}